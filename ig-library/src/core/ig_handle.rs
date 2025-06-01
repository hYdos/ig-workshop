use crate::core::ig_objects::igObject;
use crate::util::ig_name::igName;
use std::sync::{Arc, RwLock};

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
    pub object: Option<Arc<igObject>>,
}

impl igHandle {
    pub fn from_handle_name(name: &igHandleName) -> Arc<RwLock<igHandle>> {
        Arc::new(RwLock::new(igHandle {
            namespace: name.namespace.clone(),
            alias: name.name.clone(),
            object: None,
        }))
    }

    pub fn get_object_alias(&self) -> igObject {
        panic!("get_object_alias is unimplemented")
    }
}
