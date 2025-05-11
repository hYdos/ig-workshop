use crate::client::cdn::CContentDeployment;
use crate::core::ig_archive::igArchive;
use crate::core::ig_file_context::igFileContext;
use crate::core::ig_registry::{igRegistry, BuildTool};
use std::sync::Arc;

pub struct CArchive {
    pub cache_enabled: bool,
    pub do_packages: bool,
}

impl Default for CArchive {
    fn default() -> Self {
        CArchive {
            cache_enabled: false,
            do_packages: true,
        }
    }
}

impl CArchive {
    pub fn open(
        &self,
        cdn: &CContentDeployment,
        ig_file_context: &igFileContext,
        ig_registry: &igRegistry,
        path: &str,
        flags: u32,
    ) -> Result<Arc<igArchive>, String> {
        let mut res = 0;
        let mut archive_path = path.to_string();

        if (flags & 4) == 0 && !self.do_packages && !cdn.enabled {
            //res = 0;
        } else {
            if (flags & 8) == 0 {
                archive_path = get_archive_path(archive_path, ig_registry)
            }

            if self.do_packages && !cdn.enabled {
                res = 0;
            } else {
                // TODO: CStreamingInstall
            }

            if res == 0 && ((flags & 4) != 0 || self.do_packages) {
                // igCauldron sets some field in the archive before it is opened. However, these are not used and I really don't feel like messing with that atm
                let arc = Arc::new(igArchive::open(
                    ig_file_context,
                    ig_registry,
                    &archive_path,
                )?);
                if let Ok(archive_manager) = ig_file_context.archive_manager.write() {
                    archive_manager._archive_list.push(arc.clone());
                }

                return Ok(arc);
            }
        }

        Err("No criteria were met that lead to an archive been opened".to_string())
    }
}

fn get_archive_path(file_path: String, ig_registry: &igRegistry) -> String {
    match ig_registry.build_tool {
        BuildTool::AlchemyLaboratory => format!("app:/archives/{}.pak", file_path),
        BuildTool::TfbTool => file_path.to_string(), // Tfb Precache calls this, it already handles all of this
        BuildTool::None => panic!("Build tool 'None' was defined"),
    }
}
