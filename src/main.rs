#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod logger;
mod window;

use crate::logger::init_logger;
use crate::window::{igCauldronWindow, GameConfig, LoadedGame};
use eframe::HardwareAcceleration::Required;
use egui::IconData;
use egui_dock::DockState;
use ig_library::core::ig_ark_core::{igArkCore, EGame};
use ig_library::core::ig_core_platform::IG_CORE_PLATFORM;
use ig_library::core::ig_file_context::igFileContext;
use ig_library::core::ig_registry::igRegistry;
use image::ImageReader;
use log::{info, warn};
use sonic_rs::{Array, JsonContainerTrait, JsonValueTrait, Object, Value};
use std::collections::VecDeque;
use std::fs;
use std::fs::File;
use std::ops::Sub;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use std::thread::Builder;
use std::time::Instant;
use ig_library::client::c_precache_file_loader::load_init_script;

fn main() {
    init_logger();
    let configs = init_config();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_icon(icon()),
        hardware_acceleration: Required,
        ..Default::default()
    };

    eframe::run_native(
        "igWorkshop",
        options,
        Box::new(|_cc| Ok(Box::new(igCauldronWindow::new(configs)))),
    )
    .expect("How did you fail this lol");
}

pub fn load_game_data(game_cfg: GameConfig, dock_state: Arc<Mutex<DockState<window::Tab>>>) {
    Builder::new()
        .name("igGameDataLoader".to_string())
        .spawn(move || {
            let start_time = Instant::now();

            let ig_file_context = igFileContext::new(game_cfg.clone()._path);
            let mut ig_registry = igRegistry::new(game_cfg.clone()._platform);

            if !game_cfg._update_path.is_empty() {
                ig_file_context.initialize_update(&ig_registry, game_cfg.clone()._update_path);
            }
            
            let ig_ark_core = igArkCore::new(game_cfg.clone()._game);
            load_init_script(game_cfg.clone()._game, &mut ig_registry,  false);
            
            let new_leaf = Some(Arc::new(Mutex::new(LoadedGame {
                cfg: game_cfg.clone(),
                ig_file_context,
                ig_registry,
                ig_ark_core,
            })));

            // I'm going to be honest I'm not a fan of this method.
            // however, with how complex these games are we need to save performance (by not recreating tabs) as much as possible
            if let Ok(mut dock_guard) = dock_state.lock() {
                dock_guard.push_to_focused_leaf(new_leaf);
            } else { 
                panic!("We somehow failed the Mutex lock on the UI :(")
            }

            let total_time = Instant::now().sub(start_time);
            info!("Game data loaded in {:?}", total_time);
        })
        .expect("failed to spawn thread");
}

fn init_config() -> VecDeque<Arc<Mutex<GameConfig>>> {
    let cfg_path: String = String::from("_path");
    let cfg_update_path: String = String::from("_updatePath");
    let cfg_game: String = String::from("_game");
    let cfg_platform: String = String::from("_platform");
    let mut config: VecDeque<Arc<Mutex<GameConfig>>> = VecDeque::new();

    if let Some(mut path) = dirs::config_dir() {
        path.push("NefariousTechSupport");
        path.push("igCauldron");
        path.push("gameconfig.json");

        if fs::exists(path.as_path())
            .expect("Config cannot be accessed. Is something else using the file?")
        {
            info!("Reading igCauldron's gameconfig.json @ {:?}", path);
            let json_cfg: Value =
                sonic_rs::from_reader(File::open(path.as_path()).unwrap()).unwrap();
            assert_eq!(json_cfg.get("_version").unwrap(), 2);
            let games_root: &Array = json_cfg.get("_games").unwrap().as_array().unwrap();
            for x in games_root.iter() {
                let game_config: &Object = x.as_object().unwrap();
                let _game = game_config.get(&cfg_game).unwrap().to_string();
                let _platform = game_config.get(&cfg_platform).unwrap().to_string();
                config.push_back(Arc::new(Mutex::new(GameConfig {
                    _path: game_config
                        .get(&cfg_path)
                        .unwrap()
                        .to_string()
                        .replace("\"", "")
                        .replace("\\\\", "/"),
                    _update_path: game_config
                        .get(&cfg_update_path)
                        .unwrap()
                        .to_string()
                        .replace("\"", "")
                        .replace("\\\\", "/"),
                    _game: EGame::try_from(_game.replace("\"", "")).unwrap(),
                    _platform: IG_CORE_PLATFORM::try_from(_platform.replace("\"", "")).unwrap(),
                })));
            }
        }
    } else {
        warn!("Could not find config directory. New config will be saved later.");
    }

    config
}

fn icon() -> Arc<IconData> {
    let img = ImageReader::open("data/icon.png")
        .unwrap()
        .decode()
        .unwrap();
    let rgba = img.clone().as_rgba8().unwrap().to_vec();

    Arc::new(IconData {
        width: img.width(),
        height: img.height(),
        rgba,
    })
}
