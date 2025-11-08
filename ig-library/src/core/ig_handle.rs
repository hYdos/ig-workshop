use crate::core::ig_custom::igStringRefList;
use crate::core::ig_objects::{igObject, igObjectDirectory, igObjectStreamManager};
use crate::util::ig_name::igName;
use log::error;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::util::ig_hash::debug_decode_hash;

pub struct igHandleName {
    pub name: igName,
    pub namespace: igName,
}

impl igHandleName {
    pub fn new(name: igName, namespace: igName) -> Self {
        Self { name, namespace }
    }
}

#[derive(Clone)]
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

    pub fn get_object_alias(
        &mut self,
        object_stream_manager: &igObjectStreamManager,
    ) -> Option<igObject> {
        if self.object.is_some() {
            return self.object.clone();
        }

        let name_to_dir = &object_stream_manager.name_to_directory_lookup;
        if let Some(dirs) = name_to_dir.get(&self.namespace.hash) {
            for dir in dirs.iter() {
                if let Ok(dir) = dir.read() {
                    if !dir.use_name_list {
                        return None; // why? this seems weird. why not check the other igz's?
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
                "get_object_alias failed to load {}::{}",
                self.namespace
                    .string
                    .clone()
                    .unwrap_or_else(|| debug_decode_hash(self.namespace.hash)),
                self.alias
                    .string
                    .clone()
                    .unwrap_or_else(|| debug_decode_hash(self.alias.hash))
            );
            None
        }
    }
}

pub struct igObjectHandleManager {
    system_namespaces: igStringRefList,
    handle_list: Vec<u32>,
    object_to_handle_map: HashMap<usize, igHandle>,
    handle_map: HashMap<u64, igHandle>,
}

impl igObjectHandleManager {
    pub fn new() -> igObjectHandleManager {
        igObjectHandleManager {
            system_namespaces: igStringRefList::new(),
            handle_list: Vec::new(),
            object_to_handle_map: HashMap::new(),
            handle_map: HashMap::new(),
        }
    }

    pub fn add_directory(&mut self, dir: Arc<RwLock<igObjectDirectory>>) {
        if let Ok(dir) = dir.read() {
            if !dir.use_name_list {
                return;
            }

            if self.handle_list.contains(&dir.name.hash) {
                return;
            }

            self.handle_list.push(dir.name.hash);

            let object_list = dir.object_list.read().unwrap();
            let name_list = dir.name_list.read().unwrap();
            for i in 0..object_list.len() {
                let handle = self.lookup_handle(dir.name.clone(), name_list.get(i).unwrap()).clone();
                let lookup_obj = object_list.get(i).unwrap();
                self.object_to_handle_map.insert(Arc::as_ptr(&lookup_obj) as *const () as usize, handle);
            }
        }
    }

    pub fn lookup_handle_name(&mut self, name: &igHandleName) -> igHandle {
        self.lookup_handle(name.namespace.clone(), name.name.clone())
            .clone()
    }

    fn get_handle_key(ns: &igName, name: &igName) -> u64 {
        ((ns.hash as u64) << 32) | (name.hash as u64)
    }

    pub fn lookup_handle(&mut self, namespace: igName, name: igName) -> &mut igHandle {
        let key = igObjectHandleManager::get_handle_key(&namespace, &name);

        // If missing, create and insert
        let handle = self.handle_map.entry(key).or_insert_with(|| igHandle {
            namespace: namespace.clone(),
            alias: name.clone(),
            object: None,
        });

        // Attempt to set up the strings properly
        if namespace.string.is_some() && handle.namespace.string.is_none() {
            handle.namespace.string = namespace.string.clone();
        }
        if name.string.is_some() && handle.alias.string.is_none() {
            handle.alias.string = name.string.clone();
        }

        handle
    }
}
