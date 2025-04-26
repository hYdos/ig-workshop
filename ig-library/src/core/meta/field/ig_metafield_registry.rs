use crate::core::meta::field::ig_metafields::igMetaField;
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use crate::core::meta::field::r#impl::ig_placeholder_metafield::igPlaceholderMetafield;

static DEFAULT_METAFIELD_IMPL: LazyLock<Box<dyn igMetaField>> =
    LazyLock::new(|| Box::new(igPlaceholderMetafield));

/// Deals with registering implementations of MetaField and retrieving these later on
pub struct igMetafieldRegistry {
    map: HashMap<Arc<str>, Box<dyn igMetaField>>,
}

impl igMetafieldRegistry {
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl igMetafieldRegistry {
    fn register<T: Any + Send + Sync + 'static>(
        &mut self,
        name: Arc<str>,
        _impl: Box<dyn igMetaField>,
    ) {
        self.map.insert(name, _impl);
    }

    pub fn get(&self, name: &str) -> &Box<dyn igMetaField> {
        self.map.get(name).unwrap_or_else(|| { &*DEFAULT_METAFIELD_IMPL })
    }
}
