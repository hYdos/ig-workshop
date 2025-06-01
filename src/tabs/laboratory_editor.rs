use crate::window::{LoadedGame, WorkshopTabImpl, WorkshopTabViewer};
use egui::{include_image, Button, CentralPanel, Label, SidePanel, TextEdit, Ui, Vec2, Widget, WidgetText};
use egui_ltreeview::{NodeBuilder, TreeView, TreeViewBuilder};
use ig_library::core::ig_objects::{igObject, igObjectDirectory, ObjectExt};
use ig_library::util::ig_name::igName;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use log::{error, info};
use ig_library::core::ig_custom::igStringRefList;

/// Tab specifically designed for usage with games made in Vicarious Visions Laboratory.
pub struct VVLaboratoryEditor {
    game: LoadedGame,
    loaded_packages: HashMap<Arc<str>, LaboratoryPackage>,
    /// Packages that contain the search query
    filtered_packages: Vec<Arc<str>>,
    search_bar_contents: String,
    old_search_bar_contents: String
}

struct LaboratoryPackage {
    #[allow(dead_code)] // used for referencing the internal ig_object_dir if needed in the future
    pub name: igName,
    pub pkg_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub character_data_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub actor_skin_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub havok_anim_db_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub havok_rigid_body_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub havok_physics_system_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub texture_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub effect_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub shader_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub motion_path_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub igx_file_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub material_instances_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub igx_entities_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub gui_project_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub font_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub lang_file_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub spawn_mesh_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub model_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub sky_model_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub behavior_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub graph_data_behavior_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub events_behavior_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub asset_behavior_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub hkb_behavior_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub hkc_character_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub navmesh_list: Vec<Arc<RwLock<igObjectDirectory>>>,
    pub script_list: Vec<String>, // TODO: vvl impl
}

impl VVLaboratoryEditor {
    pub fn new(mut game: LoadedGame) -> Box<VVLaboratoryEditor> {
        let mut loaded_packages = HashMap::new();

        // process all initscript files loaded once at tab startup
        let loaded_igzs = &game
            .ig_alchemy
            .object_stream_manager
            .name_to_directory_lookup;
        for (_, igz_directory) in loaded_igzs {
            let objects = igz_directory.list.read().unwrap();
            for object in objects.iter() {
                let ig_object_dir = object.read().unwrap();
                let igz_name = ig_object_dir.name.string.clone().unwrap();
                let is_laboratory_package = igz_name.ends_with("_pkg.igz");
                if is_laboratory_package {
                    let user_friendly_name = igz_name
                        .replace("packages/generated/", "")
                        .replace("_pkg.igz", "");

                    loaded_packages.insert(
                        Arc::from(user_friendly_name),
                        LaboratoryPackage {
                            name: ig_object_dir.name.clone(),
                            pkg_list: vec![],
                            character_data_list: vec![],
                            actor_skin_list: vec![],
                            havok_anim_db_list: vec![],
                            havok_rigid_body_list: vec![],
                            havok_physics_system_list: vec![],
                            texture_list: vec![],
                            effect_list: vec![],
                            shader_list: vec![],
                            motion_path_list: vec![],
                            igx_file_list: vec![],
                            material_instances_list: vec![],
                            igx_entities_list: vec![],
                            gui_project_list: vec![],
                            font_list: vec![],
                            lang_file_list: vec![],
                            spawn_mesh_list: vec![],
                            model_list: vec![],
                            sky_model_list: vec![],
                            behavior_list: vec![],
                            graph_data_behavior_list: vec![],
                            events_behavior_list: vec![],
                            asset_behavior_list: vec![],
                            hkb_behavior_list: vec![],
                            hkc_character_list: vec![],
                            navmesh_list: vec![],
                            script_list: vec![],
                        },
                    );
                }
            }
        }

        let ig_registry=  &game.ig_alchemy.registry;
        let ig_file_context=  &game.ig_alchemy.file_context;
        let ig_object_stream_manager=  &mut game.ig_alchemy.object_stream_manager;
        let imm=  &mut game.ig_alchemy.ark_core.metadata_manager;
        let ig_ext_ref_system=  &mut game.ig_alchemy.ig_ext_ref_system;

        for (name, package) in &mut loaded_packages {
            let pkg_dir = ig_object_stream_manager
                .load(
                    ig_file_context,
                    ig_registry,
                    imm,
                    ig_ext_ref_system,
                    format!("packages/generated/{}_pkg.igz", name),
                )
                .unwrap();

            let guard = pkg_dir.read().unwrap();
            let ig_object_list = guard.object_list.read().unwrap();
            let objects = &ig_object_list.list.read().unwrap();
            let ig_string_ref_list = objects[0].clone().downcast::<igStringRefList>().unwrap();
            let ig_string_ref_guard = ig_string_ref_list.read().unwrap();
            let data = ig_string_ref_guard.list.read().unwrap();
            for i in (0..data.len()).step_by(2) {
                let file_data_type = &data[i];
                let file_name = data[i + 1].clone();

                match file_data_type.as_ref() {
                    "lang_file" =>  {
                        let igz = ig_object_stream_manager
                            .load(
                                ig_file_context,
                                ig_registry,
                                imm,
                                ig_ext_ref_system,
                                file_name.to_string(),
                            )
                            .unwrap();

                        package.lang_file_list.push(igz)
                    },
                    _ => error!("Unsupported data type {}", file_data_type)
                }
            }
        }

        Box::new(Self {
            game,
            loaded_packages,
            filtered_packages: vec![],
            search_bar_contents: "".to_string(),
            old_search_bar_contents: "".to_string(),
        })
    }

    /// Loops through all loaded packages and filters them based on the search bar contents.
    pub fn filter_packages(&mut self) {
        self.old_search_bar_contents = self.search_bar_contents.clone();
        self.filtered_packages.clear();
        for (package_name, _) in &self.loaded_packages {
            if package_name.contains(&self.search_bar_contents) {
                self.filtered_packages.push(package_name.clone());
            }
        }
    }

    fn render_package(builder: &mut TreeViewBuilder<u32>, package_name: Arc<str>, package: &LaboratoryPackage) {
        builder.node(
            NodeBuilder::dir(package.name.hash)
                .default_open(false)
                .activatable(true)
                .label_ui(|ui| {
                    ui.add(
                        Label::new(WidgetText::from(
                            package_name.clone().to_string(),
                        ))
                            .selectable(false),
                    );
                }),
        );

        builder.node(NodeBuilder::dir(package.name.hash + 1).label_ui(|ui| {
            ui.image(include_image!("../../data/character_data.png"));
            ui.add(Label::new("Character Data").selectable(false));
        }));
        builder.close_dir();

        builder.node(NodeBuilder::dir(package.name.hash + 2).label_ui(|ui| {
            ui.image(include_image!("../../data/actor_skins.png"));
            ui.add(Label::new("Actor Skins").selectable(false));
        }));
        builder.close_dir();

        builder.dir(package.name.hash + 3, "Havok Animation Databases");
        builder.close_dir();
        builder.dir(package.name.hash + 4, "Havok Rigid Bodies");
        builder.close_dir();
        builder.dir(package.name.hash + 5, "Havok Physics Systems");
        builder.close_dir();
        builder.dir(package.name.hash + 6, "Textures");
        builder.close_dir();
        builder.dir(package.name.hash + 7, "Effects");
        builder.close_dir();
        builder.dir(package.name.hash + 8, "Shaders");
        builder.close_dir();
        builder.dir(package.name.hash + 9, "Motion Paths");
        builder.close_dir();
        builder.dir(package.name.hash + 10, "igx Files");
        builder.close_dir();
        builder.dir(package.name.hash + 11, "Material Instances");
        builder.close_dir();
        builder.dir(package.name.hash + 12, "igx Entities");
        builder.close_dir();
        builder.dir(package.name.hash + 13, "Gui Projects");
        builder.close_dir();
        builder.dir(package.name.hash + 14, "Fonts");
        builder.close_dir();
        builder.dir(package.name.hash + 15, "Lang Files");
        builder.close_dir();
        builder.dir(package.name.hash + 16, "Spawn Meshes");
        builder.close_dir();
        builder.dir(package.name.hash + 17, "Models");
        builder.close_dir();
        builder.dir(package.name.hash + 18, "Sky Models");
        builder.close_dir();
        builder.dir(package.name.hash + 19, "Behaviours");
        builder.close_dir();
        builder.dir(package.name.hash + 20, "Graph Data");
        builder.close_dir();
        builder.dir(package.name.hash + 21, "Events Behaviours");
        builder.close_dir();
        builder.dir(package.name.hash + 22, "Asset Behaviours");
        builder.close_dir();
        builder.dir(package.name.hash + 23, "Havok Binary Behaviors");
        builder.close_dir();
        builder.dir(package.name.hash + 24, "Havok Char Characters");
        builder.close_dir();
        builder.dir(package.name.hash + 25, "Navigation Meshes");
        builder.close_dir();
        builder.dir(package.name.hash + 26, "Scripts");
        builder.close_dir();

        builder.close_dir();
    }
}
impl WorkshopTabImpl for VVLaboratoryEditor {
    fn title(&self, _viewer: &mut WorkshopTabViewer) -> WidgetText {
        format!("{} ({})", self.game.cfg._game, self.game.cfg._platform).into()
    }

    fn ui(&mut self, ui: &mut Ui, _viewer: &mut WorkshopTabViewer) {
        SidePanel::left(ui.make_persistent_id("left_file_panel"))
            .resizable(true)
            .min_width(50.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    let load_pkg = Button::new("Load Package")
                        .min_size(Vec2::new(ui.available_size_before_wrap().x, 10.0))
                        .ui(ui);

                    if load_pkg.clicked() {
                        error!("Load Package not implemented")
                    }
                });

                TextEdit::singleline(&mut self.search_bar_contents)
                    .hint_text("Search for archives")
                    .ui(ui);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let id = ui.make_persistent_id("file_tree_view");
                    TreeView::new(id).show(ui, |builder| {
                        if self.search_bar_contents.is_empty() {
                            for (package_name, package) in &self.loaded_packages {
                                VVLaboratoryEditor::render_package(builder, package_name.clone(), package);
                            }
                        } else {
                            if self.old_search_bar_contents != self.search_bar_contents {
                                self.filter_packages();
                            }

                            for package_name in &self.filtered_packages {
                                VVLaboratoryEditor::render_package(builder, package_name.clone(), self.loaded_packages.get_mut(package_name).unwrap());
                            }
                        }
                    });
                });
            });
        CentralPanel::default().show_inside(ui, |ui| {
            ui.label(format!("Content of {:?}", self.game.cfg._game));
        });
    }
}
