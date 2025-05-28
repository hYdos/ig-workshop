#![allow(non_camel_case_types)]

use crate::logger::LAST_LOG_LINE;
use crate::save_config;
use egui::{menu, InputState, Key, Ui, WidgetText};
use egui_dock::{DockArea, DockState, Style, TabViewer};
use ig_library::core::ig_ark_core::EGame;
use ig_library::core::ig_core_platform::IG_CORE_PLATFORM;
use ig_library::util::ig_common::igAlchemy;
use serde::Serialize;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, RwLock};
use crate::tabs::configuration::ConfigurationTab;

pub struct igWorkshopWindow {
    dock_state: Arc<Mutex<DockState<WorkshopTab>>>,
    tab_viewer: WorkshopTabViewer,
}

#[derive(Clone, Serialize)]
pub struct GameConfig {
    pub _path: String,
    #[serde(rename(serialize = "_updatePath"))]
    pub _update_path: String,
    pub _game: EGame,
    pub _platform: IG_CORE_PLATFORM,
}

pub struct LoadedGame {
    pub cfg: GameConfig,
    pub ig_alchemy: igAlchemy,
}

impl igWorkshopWindow {
    pub fn new(configs: VecDeque<Arc<Mutex<GameConfig>>>) -> Self {
        let dock_state = Arc::new(Mutex::new(DockState::new(vec![Box::new(ConfigurationTab) as WorkshopTab])));

        Self {
            dock_state: dock_state.clone(),
            tab_viewer: WorkshopTabViewer {
                available_games: configs,
                dock_state,
            },
        }
    }
}

impl eframe::App for igWorkshopWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        
        if ctx.input(is_save_command) {
            save(&self.tab_viewer);
        }
        
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    let save_button = ui.button("Save"); // TODO: options to save to: update.pak, overwrite game files (not recommended), or save new files to new directory)
                    let _ = ui.button("Load file");
                    let _ = ui.button("Load folder");

                    if save_button.clicked() {
                        save(&self.tab_viewer);
                    }
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

fn save(tab_viewer: &WorkshopTabViewer) {
    save_config(&tab_viewer.available_games)
}

fn is_save_command(i: &InputState) -> bool {
    i.modifiers.command && i.key_pressed(Key::S)
}

pub type WorkshopTab = Box<dyn WorkshopTabImpl + Send + Sync>;

pub struct WorkshopTabViewer {
    pub(crate) available_games: VecDeque<Arc<Mutex<GameConfig>>>,
    pub(crate) dock_state: Arc<Mutex<DockState<WorkshopTab>>>,
}

/// Allows different types of window tab types to be created under one  
pub trait WorkshopTabImpl {
    fn title(&self, viewer: &mut WorkshopTabViewer) -> WidgetText;
    fn ui(&mut self, ui: &mut Ui, viewer: &mut WorkshopTabViewer);
}

impl TabViewer for WorkshopTabViewer {
    type Tab = WorkshopTab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.as_ref().title(self)
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        tab.as_mut().ui(ui, self);
    }
}
