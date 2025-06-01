use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::core::ig_custom::igObjectList;
use crate::core::ig_handle::igHandleName;
use crate::core::ig_objects::igObject;
use crate::core::meta::ig_metadata_manager::igMetadataManager;

pub struct igExternalReferenceSystem {
    pub global_set: igReferenceResolverSet
}

impl igExternalReferenceSystem {
    pub fn new() -> Self {
        let mut external_reference_system = Self {
            global_set: igReferenceResolverSet {
                map: HashMap::new(),
            },
        };

        external_reference_system.global_set.insert("metaobject".to_string(), Arc::new(igMetaObjectReferenceResolver));
        external_reference_system.global_set.insert("metafield".to_string(), Arc::new(igMetaFieldReferenceResolver));

        external_reference_system
    }
}

struct igMetaObjectReferenceResolver;

impl igReferenceResolver for igMetaObjectReferenceResolver {
    fn resolve_reference(&self, reference: String, ctx: &mut igReferenceResolverContext<'_>) -> Option<igObject> {
        Some(ctx.ig_metadata_manager.get_or_create_meta(&reference).unwrap())
    }
}

struct igMetaFieldReferenceResolver;

impl igReferenceResolver for igMetaFieldReferenceResolver {
    fn resolve_reference(&self, name: String, ctx: &mut igReferenceResolverContext) -> Option<igObject> {
        todo!()
    }
}

pub trait igReferenceResolver: Send + Sync + 'static {
    fn resolve_reference(&self, name: String, ctx: &mut igReferenceResolverContext) -> Option<igObject>;
}

pub struct igReferenceResolverContext<'a> {
    pub root_objects: Option<Arc<RwLock<igObjectList>>>,
    pub base_path: Option<String>,
    pub data: Option<HashMap<String, igObject>>,
    pub ig_metadata_manager: &'a mut igMetadataManager
}

pub struct igReferenceResolverSet {
    map: HashMap<String, Arc<dyn igReferenceResolver>>
}

impl igReferenceResolverSet {

    pub fn resolve_reference(&self, handle_name: &igHandleName, ctx: &mut igReferenceResolverContext) -> Option<igObject> {
        if let Some(resolver) = self.map.get(&handle_name.namespace.string.clone().unwrap()) {
            resolver.resolve_reference(handle_name.name.string.clone()?, ctx)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: String, value: Arc<dyn igReferenceResolver>) {
        self.map.insert(key, value);
    }
}
