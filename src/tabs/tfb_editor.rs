use crate::window::{LoadedGame, WorkshopTabImpl, WorkshopTabViewer};
use egui::{CentralPanel, Label, SidePanel, Ui, WidgetText};
use egui_dock::{DockArea, DockState, Style, TabViewer};
use egui_ltreeview::{Action, NodeBuilder, TreeView, TreeViewBuilder};
use ig_library::core::ig_objects::igObjectDirectory;
use ig_library::util::ig_hash::hash;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::{fs, io};

pub struct TfbToolEditor {
    game: LoadedGame,
    game_files: HashMap<Arc<str>, Vec<String>>,
    selected_node: Option<String>,
    loaded_files: HashMap<Arc<str>, Arc<TfbGameFileData>>,
    dock_state: DockState<Arc<TfbGameFileData>>,
}

pub struct TfbGameFileData {
    display_name: String,
    immediate_data: Arc<RwLock<igObjectDirectory>>,
    language_data: RwLock<HashMap<Arc<str>, Arc<RwLock<igObjectDirectory>>>>,
    streamed_data: Option<Arc<RwLock<igObjectDirectory>>>,
}

pub fn load_game_content(content_dir: &str) -> io::Result<HashMap<Arc<str>, Vec<String>>> {
    let mut out: HashMap<Arc<str>, Vec<String>> = HashMap::new();

    for entry in fs::read_dir(content_dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if !file_type.is_dir() {
            continue;
        }

        let dir_path = entry.path();
        let dir_name = entry.file_name();
        let key: Arc<str> = Arc::<str>::from(dir_name.to_string_lossy().into_owned());

        let files = list_files_recursive(&dir_path)?;
        out.insert(key, files);
    }

    Ok(out)
}

fn list_files_recursive(base: &Path) -> io::Result<Vec<String>> {
    let mut stack: Vec<PathBuf> = vec![base.to_path_buf()];
    let mut files: Vec<String> = Vec::new();

    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            let ft = entry.file_type()?;
            if ft.is_file() {
                let rel = path.strip_prefix(base).unwrap_or(&path);
                files.push(rel.to_string_lossy().into_owned());
            } else if ft.is_dir() {
                stack.push(path);
            }
        }
    }

    files.sort();
    Ok(files)
}

fn render_folder(
    builder: &mut TreeViewBuilder<u32>,
    _game: &mut LoadedGame,
    folder: Arc<str>,
    game_content: &[String],
) {
    builder.node(
        NodeBuilder::dir(hash(&folder))
            .default_open(false)
            .activatable(true)
            .label_ui(|ui| {
                ui.add(Label::new(WidgetText::from(folder.clone().to_string())).selectable(false));
            }),
    );

    for file in game_content {
        if !file.ends_with(".arc") {
            builder.node(
                NodeBuilder::leaf(hash(file))
                    .default_open(false)
                    .activatable(true)
                    .label_ui(|ui| {
                        ui.add(Label::new(WidgetText::from(file[..file.rfind('.').unwrap()].to_string())).selectable(false));
                    }),
            );
        }
    }

    builder.close_dir();
}

impl TfbToolEditor {
    pub fn new(game: LoadedGame) -> Box<TfbToolEditor> {
        let mut base_game_data = load_game_content(&game.cfg._path).unwrap();

        if let Ok(update_game_data) = load_game_content(&game.cfg._update_path) {
            for (folder, game_files) in update_game_data {
                if !base_game_data.contains_key(&folder) {
                    base_game_data.insert(folder.clone(), game_files.clone());
                } else {
                    let original_game_data_folder = base_game_data.get_mut(&folder).unwrap();

                    for file in game_files {
                        if !original_game_data_folder.contains(&file) {
                            original_game_data_folder.push(file);
                        }
                    }
                }
            }
        }

        Box::new(TfbToolEditor {
            game,
            game_files: base_game_data,
            selected_node: None,
            loaded_files: HashMap::new(),
            dock_state: DockState::new(vec![]),
        })
    }

    fn get_or_load(&mut self, path: &str) -> Result<Arc<TfbGameFileData>, TfbAssetLoadError> {
        if let Some(data) = self.loaded_files.get(path) {
            Ok(data.clone())
        } else {
            let data = self.load(path)?;
            self.loaded_files.insert(Arc::from(path), Arc::new(data));
            Ok(self.loaded_files.get(path).unwrap().clone())
        }
    }

    fn load(&mut self, path: &str) -> Result<TfbGameFileData, TfbAssetLoadError> {
        let alchemy = &mut self.game.ig_alchemy;
        let file_context = &mut alchemy.file_context;
        let registry = &alchemy.registry;
        let ig_object_stream_manager = &mut alchemy.object_stream_manager;
        let ig_metadata_manager = &mut alchemy.ark_core.metadata_manager;
        let ig_external_reference_system = &mut alchemy.ig_ext_ref_system;
        let ig_object_handle_manager = &mut alchemy.ig_object_handle_manager;

        let asset_path = path.to_lowercase();

        let immediate_resource_path = &asset_path;
        let immediate_resource = file_context.load_archive(registry, immediate_resource_path);
        if let Err(reason) = immediate_resource {
            return Err(TfbAssetLoadError::IoError(format!(
                "tfb editor failed to open igArchive '{}' reason: {}",
                immediate_resource_path, reason
            )));
        }
        let immediate_resource = immediate_resource.map_err(|reason| TfbAssetLoadError::ArchiveLoadError(reason))?;

        let mut level = None;
        let mut language_igzs = HashMap::new();

        for file in &immediate_resource._files {
            let igz = ig_object_stream_manager
                .load(
                    file_context,
                    registry,
                    ig_metadata_manager,
                    ig_external_reference_system,
                    ig_object_handle_manager,
                    &format!("{}/{}", immediate_resource_path, file._name),
                )
                .map_err(|reason| {
                    TfbAssetLoadError::IgzLoadError(format!(
                        "tfb editor failed to load immediate resources. Reason: {}",
                        reason
                    ))
                })?;

            if file._name.eq("level.bld") {
                level = Some(igz);
            } else if file._name.ends_with(".pak") {
                language_igzs.insert(Arc::from(file._name.clone().replace(".pak", "").as_ref()), igz);
            } else {
                // TODO
            }
        }

        Ok(TfbGameFileData {
            display_name: path[path.rfind('/').unwrap() + 1..].to_string(),
            immediate_data: level.unwrap(),
            language_data: RwLock::new(language_igzs),
            streamed_data: None,
        })
    }
}

#[derive(Debug)]
enum TfbAssetLoadError {
    IgzLoadError(String),
    ArchiveLoadError(String),
    IoError(String)
}

struct TfbIgzEditor {

}

impl TabViewer for TfbIgzEditor {
    type Tab = Arc<TfbGameFileData>;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.display_name.clone().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {

    }
}

impl WorkshopTabImpl for TfbToolEditor {
    fn title(&self, _viewer: &mut WorkshopTabViewer) -> WidgetText {
        format!("{} ({})", self.game.cfg._game, self.game.cfg._platform).into()
    }

    fn ui(&mut self, ui: &mut Ui, _viewer: &mut WorkshopTabViewer) {
        SidePanel::left(ui.make_persistent_id("left_file_panel"))
            .resizable(true)
            .min_width(50.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let id = ui.make_persistent_id("file_tree_view");
                    let selected = TreeView::new(id).show(ui, |builder| {
                        for (folder, game_content) in &self.game_files {
                            render_folder(builder, &mut self.game, folder.clone(), game_content);
                        }
                    });

                    for action in selected.1 {
                        match action {
                            Action::Activate(activate) => {
                                for id in activate.selected {
                                    for (folder, game_files) in &self.game_files {
                                        for file in game_files {
                                            if !file.ends_with(".arc") {
                                                if hash(file).eq(&id) {
                                                    self.selected_node = Some(format!(
                                                        "{}/{}",
                                                        folder, file
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                });
            });

        CentralPanel::default().show_inside(ui, |ui| {

            DockArea::new(&mut self.dock_state)
                .style(Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut TfbIgzEditor {});

            let selected_node = self.selected_node.clone();

            if let Some(selected_node) = selected_node {
                if !self.loaded_files.contains_key::<str>(selected_node.as_ref()) {
                    println!("Loading IGZ {selected_node}");
                    match self.get_or_load(&selected_node) {
                        Ok(data) => {
                            self.dock_state.push_to_focused_leaf(data);
                        }
                        Err(TfbAssetLoadError::IgzLoadError(reason)) => {
                            panic!("igz load error {}", reason);
                        }
                        Err(TfbAssetLoadError::ArchiveLoadError(reason)) => {
                            panic!("archive load error {}", reason);
                        }
                        Err(TfbAssetLoadError::IoError(reason)) => {
                            panic!("io error {}", reason);
                        }
                    }
                }
            } else {
                ui.label("Select an archive to begin");
            }
        });
    }
}
