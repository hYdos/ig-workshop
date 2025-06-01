// TODO: on release tie this behind its own run config #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod logger;
mod tabs;
mod window;
mod plugin;

use crate::logger::init_logger;
use crate::tabs::laboratory_editor::VVLaboratoryEditor;
use crate::window::{GameConfig, LoadedGame, igWorkshopWindow};
use eframe::HardwareAcceleration::Required;
use egui::IconData;
use egui_dock::DockState;
use ig_library::client::precache::load_init_script;
use ig_library::core::ig_ark_core::{EGame, igArkCore};
use ig_library::core::ig_core_platform::IG_CORE_PLATFORM;
use ig_library::core::ig_file_context::igFileContext;
use ig_library::core::ig_registry::igRegistry;
use ig_library::util::ig_common::igAlchemy;
use image::{ImageFormat, ImageReader};
use log::{LevelFilter, error, info};
use serde::Serialize;
use sonic_rs::writer::BufferedWriter;
use sonic_rs::{Array, JsonContainerTrait, JsonValueTrait, Object, Value};
use std::collections::VecDeque;
use std::fs;
use std::fs::File;
use std::io::Cursor;
use std::ops::Sub;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use std::thread::Builder;
use std::time::Instant;

fn main() {
    #[cfg(debug_assertions)]
    init_logger(LevelFilter::Debug);
    #[cfg(not(debug_assertions))]
    init_logger(LevelFilter::Info);

    let configs = init_config();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_icon(icon()),
        hardware_acceleration: Required,
        ..Default::default()
    };

    eframe::run_native(
        "ig-workshop",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(igWorkshopWindow::new(configs)))
        }),
    )
    .expect("Failed to start ig-workshop");
}

pub fn load_game_data(
    game_cfg: GameConfig,
    dock_state: Arc<Mutex<DockState<window::WorkshopTab>>>,
) {
    Builder::new()
        .name("igGameDataLoader".to_string())
        .spawn(move || {
            let start_time = Instant::now();

            let ig_file_context = igFileContext::new(game_cfg.clone()._path);
            let ig_registry = igRegistry::new(game_cfg.clone()._platform);

            if !game_cfg._update_path.is_empty() {
                ig_file_context.initialize_update(&ig_registry, game_cfg.clone()._update_path);
            }

            let platform = ig_registry.platform.clone();
            let mut ig_alchemy = igAlchemy::new(
                ig_file_context,
                ig_registry,
                igArkCore::new(game_cfg.clone()._game, platform),
            );

            // Try out caching all metadata at the start only in debug to catch issues
            #[cfg(debug_assertions)]
            ig_alchemy.ark_core.metadata_manager.load_all();

            load_init_script(game_cfg.clone()._game, false, &mut ig_alchemy);

            let new_leaf = VVLaboratoryEditor::new(LoadedGame {
                cfg: game_cfg.clone(),
                ig_alchemy,
            });

            // I'm going to be honest I'm not a fan of this method.
            // however, with how complex these games are we need to save performance (by not recreating tabs) as much as possible
            // basically turning this into a forward-ish rendered gui instead of immediate mode in a way
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
        error!("Could not find config directory. New config will be saved later.");
    }

    config
}

#[derive(Serialize)]
struct GameConfigHeader<'a> {
    _version: u32,
    _games: &'a VecDeque<Arc<Mutex<GameConfig>>>,
}

fn save_config(game_configs: &VecDeque<Arc<Mutex<GameConfig>>>) {
    if let Some(mut path) = dirs::config_dir() {
        path.push("NefariousTechSupport");
        path.push("igCauldron");
        fs::create_dir_all(path.as_path()).unwrap();
        path.push("gameconfig.json");

        let file = File::create(path.as_path()).unwrap();
        let writer = BufferedWriter::new(file);

        sonic_rs::to_writer_pretty(
            writer,
            &GameConfigHeader {
                _version: 2,
                _games: game_configs,
            },
        )
        .unwrap();
        info!("Config saved to {}", path.as_os_str().to_str().unwrap());
    } else {
        error!("Could not save config :(");
    }
}

fn icon() -> Arc<IconData> {
    let bytes = include_bytes!("../data/icon.png");

    let img = ImageReader::with_format(Cursor::new(bytes), ImageFormat::Png)
        .decode()
        .unwrap();
    let rgba = img.clone().as_rgba8().unwrap().to_vec();

    Arc::new(IconData {
        width: img.width(),
        height: img.height(),
        rgba,
    })
}
