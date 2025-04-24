use crate::core::ig_external_ref::igExternalReferenceSystem;
use crate::core::ig_file_context::{get_native_path, igFileContext};
use crate::core::ig_lists::{igNameList, igObjectDirectoryList, igObjectList};
use crate::core::ig_registry::igRegistry;
use crate::core::load::ig_igz_loader::igIGZObjectLoader;
use crate::core::load::ig_loader;
use crate::core::load::ig_loader::igObjectLoader;
use crate::core::meta::ig_metadata_manager::{__internalObjectBase, igMetadataManager};
use crate::util::ig_hash::hash_lower;
use crate::util::ig_name::igName;
use log::warn;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type igObject = Arc<RwLock<dyn __internalObjectBase>>;

pub struct igObjectDirectory {
    pub path: String,
    pub name: igName,
    pub dependencies: igObjectDirectoryList,
    pub use_name_list: bool,
    /// List of all igObject instances present in the directory
    pub object_list: igObjectList,
    /// Only filled when use_name_list is equal to true and length should match the object list
    pub name_list: igNameList,
    pub loader: Arc<RwLock<dyn igObjectLoader>>,
}

impl igObjectDirectory {
    fn new(path: &str, name: igName) -> Self {
        igObjectDirectory {
            path: path.to_string(),
            name,
            dependencies: igObjectDirectoryList::new(),
            use_name_list: false,
            object_list: igObjectList::new(),
            name_list: igNameList::new(),
            loader: Arc::new(RwLock::new(igIGZObjectLoader)),
        }
    }
    
    /// Allows specifying a custom file loader. Handy for custom formats or formats that are not igz such as igXml, igBinary, and igAscii
    fn with_loader(path: &str, name: igName, loader: Arc<RwLock<dyn igObjectLoader>>) -> Self {
        igObjectDirectory {
            path: path.to_string(),
            name,
            dependencies: igObjectDirectoryList::new(),
            use_name_list: false,
            object_list: igObjectList::new(),
            name_list: igNameList::new(),
            loader,
        }
    }
}

pub struct igObjectStreamManager {
    pub name_to_directory_lookup: HashMap<u32, igObjectDirectoryList>,
    pub path_to_directory_lookup: HashMap<u32, Arc<RwLock<igObjectDirectory>>>,
}

impl igObjectStreamManager {
    pub fn new() -> igObjectStreamManager {
        igObjectStreamManager {
            name_to_directory_lookup: HashMap::new(),
            path_to_directory_lookup: HashMap::new(),
        }
    }

    pub fn load(
        &mut self,
        ig_file_context: &igFileContext,
        ig_registry: &igRegistry,
        ig_metadata_manager: &mut igMetadataManager,
        ig_ext_ref_system: &mut igExternalReferenceSystem,
        path: String,
    ) -> Result<Arc<RwLock<igObjectDirectory>>, String> {
        self.load_with_namespace(
            ig_file_context,
            ig_registry,
            ig_metadata_manager,
            ig_ext_ref_system,
            path.clone(),
            igName::new(path),
        )
    }

    pub fn load_with_namespace(
        &mut self,
        ig_file_context: &igFileContext,
        ig_registry: &igRegistry,
        ig_metadata_manager: &mut igMetadataManager,
        ig_ext_ref_system: &mut igExternalReferenceSystem,
        path: String,
        namespace: igName,
    ) -> Result<Arc<RwLock<igObjectDirectory>>, String> {
        let file_path = get_native_path(path);
        let file_path_hash = hash_lower(&file_path);

        if self.path_to_directory_lookup.contains_key(&file_path_hash) {
            Ok(self.path_to_directory_lookup[&file_path_hash].clone())
        } else {
            let dir = Arc::new(RwLock::new(igObjectDirectory::new(&file_path, namespace)));
            self.push_dir(dir.clone());
            let loader_result = ig_loader::get_loader(&file_path);
            if let Some(loader) = loader_result {
                let loader_guard = loader.read().unwrap();
                let mut dir_guard = dir.write().unwrap();
                loader_guard.read_file(
                    ig_file_context,
                    ig_registry,
                    self,
                    ig_ext_ref_system,
                    ig_metadata_manager,
                    &mut dir_guard,
                    &file_path,
                );
                //todo!("igObjectHandleManager.Singleton.AddDirectory(objDir);");
            } else {
                warn!("No loader found for file {}", file_path);
            }

            Ok(dir)
        }
    }

    fn push_dir(&mut self, dir: Arc<RwLock<igObjectDirectory>>) {
        let hash = dir.read().unwrap().name.hash;
        let file_path = dir.read().unwrap().path.clone();

        if !self.name_to_directory_lookup.contains_key(&hash) {
            self.name_to_directory_lookup
                .insert(hash, igObjectDirectoryList::new());
        }
        let list = self.name_to_directory_lookup.get_mut(&hash).unwrap();
        list.push(dir.clone());

        self.path_to_directory_lookup
            .insert(hash_lower(&file_path), dir);
    }
}
