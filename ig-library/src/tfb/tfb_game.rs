use crate::core::ig_external_ref::igExternalReferenceSystem;
use crate::core::ig_file_context::igFileContext;
use crate::core::ig_handle::igObjectHandleManager;
use crate::core::ig_objects::igObjectStreamManager;
use crate::core::ig_registry::igRegistry;
use crate::core::meta::ig_metadata_manager::igMetadataManager;
use std::collections::HashMap;
use std::sync::Arc;

/// Holds together both immediately loaded and streamed assets inside an asset
pub struct tfbStreamContainer {}

/// Responsible for loading game files
pub struct streamContext {}

impl streamContext {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load(
        file_context: &mut igFileContext,
        registry: &igRegistry,
        ig_object_stream_manager: &mut igObjectStreamManager,
        ig_metadata_manager: &mut igMetadataManager,
        ig_external_reference_system: &mut igExternalReferenceSystem,
        ig_object_handle_manager: &mut igObjectHandleManager,
        path: &str,
    ) -> Result<(), String> {
        let asset_path = path.to_lowercase();

        let immediate_resource_path = &asset_path;
        let immediate_resource = file_context.load_archive(registry, immediate_resource_path);
        if let Err(reason) = immediate_resource {
            return Err(format!(
                "streamContext failed to open igArchive '{}' reason: {}",
                immediate_resource_path, reason
            ));
        }
        let immediate_resource = immediate_resource?;

        // let streamed_resource_path = asset_path.replace(".bld", ".arc");
        // let streamed_resource = igArchive::open(file_context, registry, &streamed_resource_path);
        // if let Err(reason) = streamed_resource {
        //     return Err(format!(
        //         "streamContext failed to open igArchive '{}' reason: {}",
        //         streamed_resource_path, reason
        //     ));
        // }
        // let _ = streamed_resource?;

        for file in &immediate_resource._files {
            ig_object_stream_manager
                .load(
                    file_context,
                    registry,
                    ig_metadata_manager,
                    ig_external_reference_system,
                    ig_object_handle_manager,
                    &format!("{}/{}", immediate_resource_path, file._name),
                )
                .map_err(|reason| {
                    format!(
                        "streamContext failed to load immediate resources. Reason: {}",
                        reason
                    )
                })?;
        }

        Ok(())
    }
}

pub struct tfbScriptEnvironment {
    script_variables: HashMap<String, HashMap<String, u32>>,
}
pub struct tfbApplication {
    script_env: Arc<tfbScriptEnvironment>,
}

impl tfbApplication {
    pub fn open() -> tfbApplication {
        tfbApplication {
            script_env: Arc::new(tfbScriptEnvironment {
                script_variables: Default::default(),
            }),
        }
    }
}
