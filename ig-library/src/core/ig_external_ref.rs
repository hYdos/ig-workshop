use crate::core::ig_handle::igHandleName;
use crate::core::ig_objects::igObject;

pub struct igExternalReferenceSystem {
    pub global_set: igReferenceResolverSet
}

impl igExternalReferenceSystem {
    pub fn new() -> Self {
        Self {
            global_set: igReferenceResolverSet,
        }
    }
}

pub struct igReferenceResolverSet;

impl igReferenceResolverSet {

    pub fn resolve_reference(&self, _handle_name: &igHandleName) -> Option<igObject> {
        todo!()
    }
}
