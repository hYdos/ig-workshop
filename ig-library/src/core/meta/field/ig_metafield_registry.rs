use crate::core::meta::field::ig_metafields::igMetaField;
use crate::core::meta::field::r#impl::ig_placeholder_meta_field::igPlaceholderMetafield;
use crate::core::meta::ig_metadata_manager::{igMetaFieldInfo, igMetadataManager};
use crate::core::meta::ig_xml_metadata::RawArkMetaObjectField;
use log::debug;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use crate::core::ig_core_platform::IG_CORE_PLATFORM;

/// Used when you need more complex information in the meta enum
type ComplexMetaFieldFactory = fn(Arc<igMetaFieldInfo>, &igMetadataManager, &igMetafieldRegistry, IG_CORE_PLATFORM) -> Arc<dyn igMetaField>;

/// Deals with registering implementations of MetaField and retrieving these later on
pub struct igMetafieldRegistry {
    basic: HashMap<Arc<str>, Arc<dyn igMetaField>>,
    complex: HashMap<Arc<str>, ComplexMetaFieldFactory>,
}

impl igMetafieldRegistry {
    pub(crate) fn new() -> Self {
        Self {
            basic: HashMap::new(),
            complex: HashMap::new(),
        }
    }
}

impl igMetafieldRegistry {
    pub fn register<T: Any + Send + Sync + 'static>(
        &mut self,
        name: Arc<str>,
        _impl: Arc<dyn igMetaField>,
    ) {
        self.basic.insert(name, _impl);
    }

    /// Used on metafields that need more specific information to function correctly.
    pub fn register_complex<T: Any + Send + Sync + 'static>(
        &mut self,
        name: Arc<str>,
        _impl: ComplexMetaFieldFactory,
    ) {
        self.complex.insert(name, _impl);
    }

    pub fn get(&self, field: Arc<igMetaFieldInfo>, imm: &igMetadataManager, platform: IG_CORE_PLATFORM) -> Arc<dyn igMetaField> {
        let type_name = &field._type.clone();

        match self.basic.get(type_name) {
            Some(v) => v.clone(),
            None => match self.complex.get(type_name) {
                Some(v) => v(field.clone(), imm, self, platform).clone(),
                None => {
                    debug!(
                        "instantiated a new igPlaceholderMetafield. No implementation for {}",
                        field._type
                    );
                    Arc::new(igPlaceholderMetafield(
                        field.size,
                        field.name.clone().unwrap(),
                    ))
                }
            },
        }
    }

    /// Used to get inner types which are known not to need fancy features.
    pub(crate) fn get_simple(&self, ark_info: &RawArkMetaObjectField) -> Arc<dyn igMetaField> {
        let name = &ark_info._type.clone();
        match self.basic.get(name) {
            Some(v) => v.clone(),
            None => {
                debug!("instantiated a new igPlaceholderMetafield. No SIMPLE implementation for {}. (This could be very bad lots of my code would need to be changed to make this work pls tell me it's not a complex type)", name);
                Arc::new(igPlaceholderMetafield(
                    ark_info.required_alignment.unwrap() as u32,
                    ark_info.name.clone().unwrap(),
                ))
            }
        }
    }
}
