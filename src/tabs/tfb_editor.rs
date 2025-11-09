use crate::window::{LoadedGame, WorkshopTabImpl, WorkshopTabViewer};
use egui::{CentralPanel, Label, SidePanel, Ui, WidgetText};
use egui_dock::{DockArea, DockState, Style, TabViewer};
use egui_ltreeview::{Action, NodeBuilder, TreeView, TreeViewBuilder};
use ig_library::core::ig_custom::igObjectList;
use ig_library::core::ig_objects::{ObjectExt, igAny, igObject, igObjectDirectory};
use ig_library::core::memory::igMemory;
use ig_library::core::meta::ig_metadata_manager::{
    igMetaFieldInfo, igMetaObject, igMetadataManager,
};
use ig_library::util::ig_common::igAlchemy;
use ig_library::util::ig_hash::hash;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock, RwLockReadGuard};
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
                        ui.add(
                            Label::new(WidgetText::from(
                                file[..file.rfind('.').unwrap()].to_string(),
                            ))
                            .selectable(false),
                        );
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
        let immediate_resource =
            immediate_resource.map_err(|reason| TfbAssetLoadError::ArchiveLoadError(reason))?;

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
                language_igzs.insert(
                    Arc::from(file._name.clone().replace(".pak", "").as_ref()),
                    igz,
                );
            } else {
                // TODO
            }
        }

        Ok(TfbGameFileData {
            display_name: path[path.rfind('/').unwrap() + 1..path.rfind('.').unwrap()].to_string(),
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
    IoError(String),
}

struct TfbIgzEditor<'a> {
    alchemy: &'a mut igAlchemy,
}

impl TfbIgzEditor<'_> {
    fn render_ig_object(
        &mut self,
        meta: &igMetaObject,
        builder: &mut TreeViewBuilder<u32>,
        object: igObject,
        object_list: Arc<RwLock<igObjectList>>,
        name: String,
        id: String,
        idx: usize,
    ) {
        if "igObjectList".eq(meta.name.as_ref()) {
            let list = object.clone().downcast::<igObjectList>().unwrap();
            if Arc::as_ptr(&object_list) != Arc::as_ptr(&list) {
                builder.node(
                    NodeBuilder::dir(hash(&id))
                        .default_open(false)
                        .activatable(true)
                        .label_ui(|ui| {
                            ui.add(Label::new(WidgetText::from(&name)).selectable(false));
                        }),
                );

                builder.node(
                    NodeBuilder::leaf(hash(&id) + 69420)
                        .default_open(false)
                        .activatable(false)
                        .label_ui(|ui| {
                            ui.add(
                                Label::new(WidgetText::from(format!(
                                    "Big Pain to deal with. Bug me later. Size is {}",
                                    list.read().unwrap().len()
                                )))
                                .selectable(false),
                            );
                        }),
                );
            }
        } else {
            builder.node(
                NodeBuilder::dir(hash(&id))
                    .default_open(false)
                    .activatable(true)
                    .label_ui(|ui| {
                        ui.add(Label::new(WidgetText::from(&name)).selectable(false));
                    }),
            );

            for (field_name, info) in &meta.field_storage.name_lookup {
                let id = format!("immediate_resources/{idx}/{field_name}");
                let value = object.read().unwrap().get_field(field_name).unwrap();

                // some types need extra context, so we have to define them here.
                match info._type.as_ref() {
                    "igObjectRefMetaField" => {
                        if let Some(value) = value {
                            let ark_info = info.ark_info.read().unwrap();
                            let guard = value.read().unwrap();
                            let referenced_object = guard.downcast_ref::<igObject>().unwrap();

                            let referenced_object_type = ark_info.meta_object.clone().unwrap();
                            let referenced_object_meta = self
                                .alchemy
                                .ark_core
                                .metadata_manager
                                .get_or_create_meta(referenced_object_type.as_ref())
                                .unwrap();

                            self.render_ig_object(
                                &referenced_object_meta.read().unwrap(),
                                builder,
                                referenced_object.clone(),
                                object_list.clone(),
                                format!(
                                    "{} (igObjectRefMetaField of {}) ",
                                    info.name.clone().unwrap(),
                                    referenced_object_type
                                ),
                                id.to_string(),
                                i32::MAX as usize,
                            )
                        }
                    }
                    _ => {
                        builder.node(
                            NodeBuilder::leaf(hash(&id))
                                .default_open(false)
                                .activatable(false)
                                .label_ui(|ui| {
                                    ui.add(
                                        Label::new(WidgetText::from(field_name.as_ref()))
                                            .selectable(false),
                                    );

                                    match &value {
                                        None => {
                                            ui.add(
                                                Label::new(WidgetText::from("(null)"))
                                                    .selectable(false),
                                            );
                                        }
                                        Some(value) => {
                                            TfbIgzEditor::render_field_value(
                                                ui,
                                                &mut self.alchemy.ark_core.metadata_manager,
                                                info,
                                                value,
                                                &meta,
                                            );
                                        }
                                    }
                                }),
                        );
                    }
                }
            }
        }

        builder.close_dir();
    }

    fn render_igz(
        &mut self,
        id_prefix: &str,
        builder: &mut TreeViewBuilder<u32>,
        igz: &Arc<RwLock<igObjectDirectory>>,
    ) {
        if let Ok(object_dir) = igz.read() {
            let object_list = object_dir.object_list.read().unwrap();
            for i in 0..object_list.len() {
                let id = format!("{id_prefix}/{i}");
                let object = object_list.get(i).unwrap();
                let object_guard = object.read().unwrap();
                let name = match object_guard.get_field("_name") {
                    Ok(Some(name)) => name
                        .read()
                        .unwrap()
                        .downcast_ref::<Arc<str>>()
                        .unwrap()
                        .to_string(),
                    _ => {
                        format!("igObject {}", i)
                    }
                };
                let name = format!("{name} ({})", object_guard.object_name());

                let meta = object_guard.meta_type(&mut self.alchemy.ark_core.metadata_manager);
                drop(object_guard);
                if let Ok(meta) = meta.read() {
                    self.render_ig_object(
                        &meta,
                        builder,
                        object,
                        object_dir.object_list.clone(),
                        name,
                        id.clone(),
                        i,
                    );
                }
            }
        }
    }

    fn render_field_value(
        ui: &mut Ui,
        ig_metadata_manager: &mut igMetadataManager,
        info: &Arc<igMetaFieldInfo>,
        value: &igAny,
        object_meta: &igMetaObject,
    ) {
        match info._type.as_ref() {
            "igStringMetaField" => {
                ui.add(
                    Label::new(WidgetText::from(format!(
                        "{}",
                        value.read().unwrap().downcast_ref::<Arc<str>>().unwrap()
                    )))
                    .selectable(false),
                );
            }
            "igIntMetaField" => {
                ui.add(
                    Label::new(WidgetText::from(format!(
                        "{}",
                        value.read().unwrap().downcast_ref::<i32>().unwrap()
                    )))
                    .selectable(false),
                );
            }
            "igFloatMetaField" => {
                ui.add(
                    Label::new(WidgetText::from(format!(
                        "{}",
                        value.read().unwrap().downcast_ref::<f32>().unwrap()
                    )))
                    .selectable(false),
                );
            }
            "igBoolMetaField" => {
                ui.add(
                    Label::new(WidgetText::from(format!(
                        "{}",
                        value.read().unwrap().downcast_ref::<bool>().unwrap()
                    )))
                    .selectable(false),
                );
            }
            "igMemoryRefMetaField" => {
                let weak_ig_memory = value.read().unwrap();
                let ig_memory = weak_ig_memory.downcast_ref::<igMemory<igAny>>().unwrap();
                let memory_ref_info = info
                    .ark_info
                    .read()
                    .unwrap()
                    .clone()
                    .ig_memory_ref_info
                    .unwrap();
                let internal_ark_object = memory_ref_info.read().unwrap();
                let meta = ig_metadata_manager
                    .get_or_create_meta(&internal_ark_object._type)
                    .unwrap();

                println!(
                    "igObject is of type {}. igMemory has {} objects. This is from field {} in object {}",
                    meta.read().unwrap().name,
                    ig_memory.data.len(),
                    info.name.clone().unwrap(),
                    object_meta.name
                );

                for entry in &ig_memory.data {}
                // let memory_ref_info = info.ark_info.read().unwrap().clone().ig_memory_ref_info.unwrap();
                // let guard = memory_ref_info.read().unwrap();
                // self.render_field_value(ui, Arc::new(igMetaFieldInfo {
                //     ark_info: memory_ref_info,
                //     _type: guard._type.clone(),
                //     name: guard.name.clone(),
                //     size: 0,
                //     alignment: 0,
                //     offset: 0,
                // }), value)
            }
            _ => {
                ui.add(
                    Label::new(WidgetText::from(format!(
                        "no implementation for {}",
                        info._type
                    )))
                    .selectable(false),
                );
            }
        }
    }
}

impl TabViewer for TfbIgzEditor<'_> {
    type Tab = Arc<TfbGameFileData>;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.display_name.clone().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        TreeView::new(ui.make_persistent_id(tab.display_name.clone())).show(ui, |mut builder| {
            builder.node(
                NodeBuilder::dir(hash("immediate_resources"))
                    .default_open(false)
                    .activatable(true)
                    .label_ui(|ui| {
                        ui.add(
                            Label::new(WidgetText::from("Level Data (level.bld)"))
                                .selectable(false),
                        );
                    }),
            );
            self.render_igz("immediate_resources", &mut builder, &tab.immediate_data);
            builder.close_dir();

            builder.node(
                NodeBuilder::dir(hash("language_resources"))
                    .default_open(false)
                    .activatable(true)
                    .label_ui(|ui| {
                        ui.add(
                            Label::new(WidgetText::from("Language Data (e.g ENGLISH.pak)"))
                                .selectable(false),
                        );
                    }),
            );
            let language_map = tab.language_data.read().unwrap();
            for key in language_map.keys() {
                let id = format!("language_resources/{key}");
                builder.node(
                    NodeBuilder::dir(hash(&id))
                        .default_open(false)
                        .activatable(true)
                        .label_ui(|ui| {
                            ui.add(Label::new(WidgetText::from(key.as_ref())).selectable(false));
                        }),
                );

                let igz = language_map.get(key).unwrap();
                self.render_igz(&id, &mut builder, &igz);
                builder.close_dir();
            }
            builder.close_dir();

            builder.node(
                NodeBuilder::dir(hash("streamed_resources"))
                    .default_open(false)
                    .activatable(true)
                    .label_ui(|ui| {
                        ui.add(
                            Label::new(WidgetText::from("Streamed Data (.arc resources)"))
                                .selectable(false),
                        );
                    }),
            );
            builder.close_dir();
        });
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
                                                    self.selected_node =
                                                        Some(format!("{}/{}", folder, file));
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
            let id = ui.make_persistent_id("igz_file_dock");
            let mut style = Style::from_egui(ui.style().as_ref());
            style.dock_area_padding = None;

            DockArea::new(&mut self.dock_state)
                .id(id)
                .window_bounds(ui.max_rect())
                .style(style)
                .show_inside(
                    ui,
                    &mut TfbIgzEditor {
                        alchemy: &mut self.game.ig_alchemy,
                    },
                );

            let selected_node = self.selected_node.clone();

            if let Some(selected_node) = selected_node {
                if !self
                    .loaded_files
                    .contains_key::<str>(selected_node.as_ref())
                {
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
            }
        });
    }
}
