use crate::core::ig_file_context::igFileContext;
use crate::core::ig_objects::{igObjectDirectory, igObjectStreamManager};
use crate::core::ig_registry::igRegistry;
use crate::core::load::ig_igz_loader::igIGZObjectLoader;
use crate::core::meta::ig_metadata_manager::igMetadataManager;
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};
use crate::core::external_ref::igExternalReferenceSystem;

static LOADERS: Lazy<[Arc<RwLock<dyn igObjectLoader>>; 1]> =
    Lazy::new(|| [Arc::new(RwLock::new(igIGZObjectLoader))]);

/// The shared base between anything that can load an alchemy binary (igz, igx, igb)
pub trait igObjectLoader: Send + Sync {
    /// Returns true if the loader can load the specified file
    fn can_read(&self, file_name: &str) -> bool;

    /// Internal name of the loader.
    fn get_name(&self) -> &'static str;

    /// The provider of the loader. For the built-in loaders of alchemy, this will usually be "Alchemy"
    fn get_type(&self) -> &'static str;

    fn read_file(
        &self,
        ig_file_context: &igFileContext,
        ig_registry: &igRegistry,
        ig_object_stream_manager: &mut igObjectStreamManager,
        ig_ext_ref_system: &mut igExternalReferenceSystem,
        ig_metadata_manager: &mut igMetadataManager,
        dir: &mut igObjectDirectory,
        file_path: &str,
    );
}

pub fn get_loader(file_path: &str) -> Option<Arc<RwLock<dyn igObjectLoader>>> {
    for loader in LOADERS.iter() {
        let loader_guard = loader.read().unwrap();
        if loader_guard.can_read(file_path) {
            drop(loader_guard);
            return Some(loader.clone());
        }
    }

    None
}
