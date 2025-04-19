#![allow(non_camel_case_types)]

use crate::load_game_data;
use crate::logger::LAST_LOG_LINE;
use egui::{menu, CollapsingHeader, Ui, WidgetText};
use egui_dock::{DockArea, DockState, Style, TabViewer};
use ig_library::core::ig_ark_core::EGame;
use ig_library::core::ig_core_platform::IG_CORE_PLATFORM;
use log::error;
use std::collections::VecDeque;
use std::path::Path;
use std::sync::{Arc, Mutex};
use ig_library::util::ig_common::igAlchemy;

pub struct igCauldronWindow {
    dock_state: Arc<Mutex<DockState<Tab>>>,
    tab_viewer: CauldronTabViewer,
}

#[derive(Clone)]
pub struct GameConfig {
    pub _path: String,
    pub _update_path: String,
    pub _game: EGame,
    pub _platform: IG_CORE_PLATFORM,
}

pub struct LoadedGame {
    pub cfg: GameConfig,
    pub ig_alchemy: igAlchemy
}

pub(crate) type Tab = Option<Arc<Mutex<LoadedGame>>>;

impl igCauldronWindow {
    pub fn new(configs: VecDeque<Arc<Mutex<GameConfig>>>) -> Self {
        let dock_state = Arc::new(Mutex::new(DockState::new(vec![None])));

        Self {
            dock_state: dock_state.clone(),
            tab_viewer: CauldronTabViewer {
                available_games: configs,
                dock_state,
            },
        }
    }
}

impl eframe::App for igCauldronWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    let _ = ui.button("Save"); // TODO: options to save to: update.pak, overwrite game files (not recommended), or save new files to new directory)
                    let _ = ui.button("Load file");
                    let _ = ui.button("Load folder");
                });
            });
        });
        egui::TopBottomPanel::bottom("log_info").show(ctx, |ui| {
            ui.label(LAST_LOG_LINE.lock().unwrap().clone());
        });

        let mut dock_guard = self.dock_state.lock().unwrap();

        DockArea::new(&mut dock_guard)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.tab_viewer);
    }
}

struct CauldronTabViewer {
    available_games: VecDeque<Arc<Mutex<GameConfig>>>,
    dock_state: Arc<Mutex<DockState<Option<Arc<Mutex<LoadedGame>>>>>>,
}

impl TabViewer for CauldronTabViewer {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        if tab.is_some() {
            let locked_cfg = tab.clone().unwrap();
            let game = locked_cfg.lock().unwrap();
            format!("{} ({})", game.cfg._game, game.cfg._platform).into()
        } else {
            "Configuration".into()
        }
    }

    // Defines the contents of a given `tab`.
    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        if tab.is_some() {
            let locked_cfg = tab.clone().unwrap();
            let config = locked_cfg.lock().unwrap();
            let game = &config.cfg._game;
            ui.label(format!("Content of {:?}", game));
        } else {
            let mut game_cfg_to_remove: Option<usize> = None;

            for game_idx in 0..self.available_games.len() {
                let mut game_cfg = self.available_games.get(game_idx).unwrap().lock().unwrap();

                CollapsingHeader::new(format!("{} ({})", game_cfg._game, game_cfg._platform))
                    .id_salt(game_idx)
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            ui.label("Target Game");
                            egui::ComboBox::from_id_salt("Target Game")
                                .selected_text(format!("{}", game_cfg._game))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_MadagascarTMEscape2AfricaTMTheGameTM, format!("{}", EGame::EV_MadagascarTMEscape2AfricaTMTheGameTM));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersSpyrosAdventure, format!("{}", EGame::EV_SkylandersSpyrosAdventure));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersSpyrosAdventure_3DS, format!("{}", EGame::EV_SkylandersSpyrosAdventure_3DS));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_HatsuneMikuProjectDiva, format!("{}", EGame::EV_HatsuneMikuProjectDiva));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_HatsuneMikuProjectDiva2nd, format!("{}", EGame::EV_HatsuneMikuProjectDiva2nd));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_HatsuneMikuProjectDivaExtend, format!("{}", EGame::EV_HatsuneMikuProjectDivaExtend));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersBattlegrounds, format!("{}", EGame::EV_SkylandersBattlegrounds));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersCloudPatrol, format!("{}", EGame::EV_SkylandersCloudPatrol));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersGiants, format!("{}", EGame::EV_SkylandersGiants));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersGiants_3DS, format!("{}", EGame::EV_SkylandersGiants_3DS));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersLostIslands, format!("{}", EGame::EV_SkylandersLostIslands));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersSwapForce, format!("{}", EGame::EV_SkylandersSwapForce));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersSwapForce_3DS, format!("{}", EGame::EV_SkylandersSwapForce_3DS));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersTrapTeam, format!("{}", EGame::EV_SkylandersTrapTeam));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersTrapTeam_3DS, format!("{}", EGame::EV_SkylandersTrapTeam_3DS));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersSuperchargers, format!("{}", EGame::EV_SkylandersSuperchargers));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersSuperchargersIos, format!("{}", EGame::EV_SkylandersSuperchargersIos));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersImaginators, format!("{}", EGame::EV_SkylandersImaginators));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_SkylandersImaginatorsSwitch, format!("{}", EGame::EV_SkylandersImaginatorsSwitch));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_CrashNSaneTrilogy, format!("{}", EGame::EV_CrashNSaneTrilogy));
                                    ui.selectable_value(&mut game_cfg._game, EGame::EV_CrashTeamRacingNitroFueled, format!("{}", EGame::EV_CrashTeamRacingNitroFueled));
                                });
                        });

                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            ui.label("Target Platform");
                            egui::ComboBox::from_id_salt("Target Platform")
                                .selected_text(format!("{}", game_cfg._platform))
                                .show_ui(ui, |ui| {
                                    if game_cfg._game != EGame::EV_SkylandersSuperchargersIos {
                                    }

                                    if game_cfg._game != EGame::EV_SkylandersImaginatorsSwitch {
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_ASPEN64, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_ASPEN64));
                                    }

                                    if game_cfg._game != EGame::EV_SkylandersImaginatorsSwitch && game_cfg._game != EGame::EV_SkylandersSuperchargersIos {
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_WIN32, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_WIN32));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_WII, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_WII));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_DURANGO, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_DURANGO));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_ASPEN, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_ASPEN));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_XENON, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_XENON));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_PS3, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_PS3));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_OSX, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_OSX));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_WIN64, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_WIN64));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_CAFE, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_CAFE));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_NGP, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_NGP));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_MARMALADE, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_MARMALADE));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_RASPI, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_RASPI));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_ANDROID, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_ANDROID));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_LGTV, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_LGTV));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_PS4, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_PS4));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_WP8, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_WP8));
                                        ui.selectable_value(&mut game_cfg._platform, IG_CORE_PLATFORM::IG_CORE_PLATFORM_LINUX, format!("{}", IG_CORE_PLATFORM::IG_CORE_PLATFORM_LINUX));
                                    }
                                });
                        });

                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            ui.label("Game Path");
                            ui.text_edit_singleline(&mut game_cfg._path);
                            let _ = ui.button("Browse"); // TODO:
                        });

                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            ui.label("update.pak Path");
                            ui.text_edit_singleline(&mut game_cfg._update_path);
                            let _ = ui.button("Browse"); // TODO:
                        });

                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            let load_game = ui.button("Load Game"); // TODO: disable this once loaded. allow a unload game and open game option once loaded
                            // debug game like feature in the future?

                            if load_game.clicked() {
                                let has_update = !game_cfg._update_path.is_empty();
                                if has_update && !Path::exists(game_cfg._update_path.clone().as_ref()) {
                                    error!("Your update.pak directory is not empty, but we can't find the file.")
                                }

                                if !Path::exists(game_cfg._path.clone().as_ref()) {
                                    error!("Game path does not exist");
                                }

                                if game_cfg._platform == IG_CORE_PLATFORM::IG_CORE_PLATFORM_DEPRECATED {
                                    error!("Invalid Platform");
                                }

                                load_game_data(game_cfg.clone(), self.dock_state.clone());
                            }
                        });


                        if ui.button("delete game config").clicked() {
                            game_cfg_to_remove = Some(game_idx)
                        }
                    });
            }

            if game_cfg_to_remove.is_some() {
                self.available_games.remove(game_cfg_to_remove.unwrap());
            }

            if ui.button("Add Game").clicked() {
                self.available_games
                    .push_back(Arc::new(Mutex::new(GameConfig {
                        _path: "".to_string(),
                        _update_path: "".to_string(),
                        _game: EGame::EV_None,
                        _platform: IG_CORE_PLATFORM::IG_CORE_PLATFORM_DEFAULT,
                    })))
            }
        }
    }
}
