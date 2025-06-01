use crate::core::ig_fs::Endian;
use crate::core::ig_objects::igAny;
use crate::core::load::ig_igb_loader::IgbLoaderContext;
use crate::core::load::ig_igx_loader::IgxLoaderContext;
use crate::core::load::ig_igz_loader::IgzLoaderContext;
use crate::core::meta::field::ig_metafield_registry::igMetafieldRegistry;
use crate::core::meta::field::ig_metafields::igMetaField;
use crate::core::meta::ig_metadata_manager::igMetadataManager;
use crate::core::save::ig_igb_saver::{IgbSaverContext, IgbSaverError};
use crate::core::save::ig_igx_saver::{IgxSaverContext, IgxSaverError};
use crate::core::save::ig_igz_saver::{IgzSaverContext, IgzSaverError};
use crate::util::byteorder_fixes::{read_ptr, read_string};
use std::any::TypeId;
use std::io::Cursor;
use std::sync::{Arc, RwLock};

pub struct igStringMetaField;

impl igMetaField for igStringMetaField {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Arc<str>>()
    }

    fn value_from_igz(
        &self,
        _registry: &igMetafieldRegistry,
        _metadata_manager: &igMetadataManager,
        handle: &mut Cursor<Vec<u8>>,
        endian: Endian,
        ctx: &IgzLoaderContext,
    ) -> Option<igAny> {
        let base_pos = handle.position();
        let is_ref = ctx
            .runtime_fields
            .string_references
            .binary_search(&base_pos).is_ok();

        let is_table = ctx.runtime_fields.string_tables.binary_search(&base_pos) >= Ok(0);

        let raw = read_ptr(handle, ctx.platform.clone(), endian).unwrap();
        let mut result: Option<String> = None;

        if is_ref {
            let offset = ctx.deserialize_offset(raw);
            handle.set_position(offset);
            result = Some(read_string(handle).unwrap());
        } else if is_table {
            result = Some(ctx.string_list[raw as usize].clone());
        }

        handle.set_position(base_pos + ctx.platform.get_pointer_size() as u64);
        // Based rust casting
        result.map(|s| {
            // 1) make an Arc<str>
            let arc_str: Arc<str> = Arc::from(s.into_boxed_str());
            // 2) lock the Arc<str>, producing Arc<RwLock<Arc<str>>>
            let concrete: Arc<RwLock<Arc<str>>> = Arc::new(RwLock::new(arc_str));
            // 3) coerce to igAny
            let object: igAny = concrete;
            object
        })
    }

    fn value_into_igz(
        &self,
        _registry: &igMetafieldRegistry,
        _metadata_manager: &igMetadataManager,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: Endian,
        _ctx: &mut IgzSaverContext,
    ) -> Result<(), IgzSaverError> {
        todo!()
    }

    fn value_from_igx(
        &self,
        _registry: &igMetafieldRegistry,
        _metadata_manager: &igMetadataManager,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: Endian,
        _ctx: &mut IgxLoaderContext,
    ) -> Option<igAny> {
        todo!()
    }

    fn value_into_igx(
        &self,
        _registry: &igMetafieldRegistry,
        _metadata_manager: &igMetadataManager,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: Endian,
        _ctx: &mut IgxSaverContext,
    ) -> Result<(), IgxSaverError> {
        todo!()
    }

    fn value_from_igb(
        &self,
        _registry: &igMetafieldRegistry,
        _metadata_manager: &igMetadataManager,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: Endian,
        _ctx: &mut IgbLoaderContext,
    ) -> Option<igAny> {
        todo!()
    }

    fn value_into_igb(
        &self,
        _registry: &igMetafieldRegistry,
        _metadata_manager: &igMetadataManager,
        _handle: &mut Cursor<Vec<u8>>,
        _endian: Endian,
        _ctx: &mut IgbSaverContext,
    ) -> Result<(), IgbSaverError> {
        todo!()
    }
}
