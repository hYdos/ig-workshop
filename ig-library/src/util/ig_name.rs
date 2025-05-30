use crate::core::meta::field::r#impl::ig_string_meta_field::igStringMetaField;
use std::sync::Arc;
use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_fs::Endian;
use crate::core::ig_objects::igAny;
use crate::core::load::ig_igb_loader::IgbLoaderContext;
use crate::core::load::ig_igx_loader::IgxLoaderContext;
use crate::core::load::ig_igz_loader::IgzLoaderContext;
use crate::core::meta::field::ig_metafield_registry::igMetafieldRegistry;
use crate::core::meta::field::ig_metafields::igMetaField;
use crate::core::save::ig_igb_saver::{IgbSaverContext, IgbSaverError};
use crate::core::save::ig_igx_saver::{IgxSaverContext, IgxSaverError};
use crate::core::save::ig_igz_saver::{IgzSaverContext, IgzSaverError};
use crate::util::ig_hash::hash_lower;
use ig_proc_macros::igStruct;
use crate::core::meta::ig_metadata_manager::igMetadataManager;

#[igStruct]
#[derive(Debug, Clone)]
pub struct igName {
    pub string: Option<String>,
    pub hash: u32,
}

impl igName {
    pub fn new(string: String) -> Self {
        igName {
            hash: hash_lower(&string),
            string: Some(string),
        }
    }

    pub fn from_hash(hash: u32) -> Self {
        igName { hash, string: None }
    }
}
