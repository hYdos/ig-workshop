use crate::core::ig_objects::{igObject, igObjectStreamManager};
use crate::util::ig_name::igName;
use std::sync::{Arc, RwLock};
use log::error;

pub struct igHandleName {
    pub name: igName,
    pub namespace: igName,
}

impl igHandleName {
    pub fn new(name: igName, namespace: igName) -> Self {
        Self { name, namespace }
    }
}

pub struct igHandle {
    pub namespace: igName,
    pub alias: igName,
    pub object: Option<igObject>,
}

impl igHandle {
    pub fn from_handle_name(name: &igHandleName) -> Arc<RwLock<igHandle>> {
        Arc::new(RwLock::new(igHandle {
            namespace: name.namespace.clone(),
            alias: name.name.clone(),
            object: None,
        }))
    }

    pub fn get_object_alias(&mut self, object_stream_manager: &igObjectStreamManager) -> Option<igObject> {
        if self.object.is_some() {
            return self.object.clone();
        }

        let name_to_dir = &object_stream_manager.name_to_directory_lookup;
        if let Some(dirs) = name_to_dir.get(&self.namespace.hash) {
            for dir in dirs.iter() {
                if let Ok(dir) = dir.read() {
                    if !dir.use_name_list {
                        return None // why? this seems weird. why not check the other igz's?
                    }

                    let name_list = dir.name_list.read().unwrap();
                    for i in 0..name_list.len() {
                        let handle = name_list.get(i).unwrap();
                        if handle.hash == self.alias.hash {
                            self.object = dir.object_list.read().unwrap().get(i)
                        }
                    }
                }
            }
            
            None
        } else {
            error!(
            "get_object_alias failed to load {}.{}",
            self.namespace.string.clone().unwrap_or_else(move || "(null)".to_string()),
            self.alias.string.clone().unwrap_or_else(move || "(null)".to_string())
        );
            None
        }
    }
}
