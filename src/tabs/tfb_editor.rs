use crate::window::{LoadedGame, WorkshopTabImpl, WorkshopTabViewer};
use egui::{CentralPanel, Label, SidePanel, Ui, WidgetText};
use egui_ltreeview::{Action, NodeBuilder, TreeView, TreeViewBuilder};
use ig_library::util::ig_hash::hash;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fs, io};

pub struct TfbToolEditor {
    game: LoadedGame,
    game_files: HashMap<Arc<str>, Vec<String>>,
    selected_node: Option<String>,
}

// TODO: handle update content overlaying
pub fn load_game_content(content_dir: &str) -> io::Result<HashMap<Arc<str>, Vec<String>>> {
    let mut out: HashMap<Arc<str>, Vec<String>> = HashMap::new();

    // Iterate first-level entries under content/
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
                        ui.add(Label::new(WidgetText::from(file)).selectable(false));
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
        })
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
                                                        "{}/{}/level.bld",
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
            if let Some(selected_node) = &self.selected_node {
                if let Some(igz) = self.game.ig_alchemy.get_if_loaded(selected_node.clone()) {
                    match igz.read() {
                        Ok(ig_obj_dir) => {
                            let object_list = ig_obj_dir.object_list.read().unwrap();
                            let name_list = ig_obj_dir.name_list.read().unwrap();
                            
                            if object_list.len() == 0 {
                                ui.label("Empty IGZ");
                            }
                            
                            for i in 0..object_list.len() {
                                let name = match ig_obj_dir.use_name_list {
                                    true => format!(
                                        "{} (Object {})",
                                        name_list.get(i).unwrap().string.unwrap(),
                                        i
                                    ),

                                    false => {
                                        let object = object_list.get(i).unwrap();
                                        let object = object.read().unwrap();

                                        if let Ok(Some(name)) = object.get_field("_name") {
                                            format!(
                                                "Object {} {} ({})",
                                                i,
                                                name.read()
                                                    .unwrap()
                                                    .downcast_ref::<Arc<str>>()
                                                    .unwrap(),
                                                object.object_name()
                                            )
                                        } else {
                                            format!("Object {}", i)
                                        }
                                    }
                                };

                                ui.label(name);
                            }
                        }
                        Err(_) => {
                            ui.label("locking IGZ for reading failed :(");
                        }
                    }
                }
            } else {
                ui.label("Select an archive to begin");
            }
        });
    }
}
