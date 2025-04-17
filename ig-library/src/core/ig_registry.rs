use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::gfx::ig_gfx_platform::IG_GFX_PLATFORM;

pub enum BuildTool {
    AlchemyLaboratory,
    TfbTool,
    None,
}

pub struct igRegistry {
    /// The build tools used to build the target game. This information is set after the init script is read.
    pub build_tool: BuildTool,
    pub platform: IG_CORE_PLATFORM,
    pub gfx_platform: IG_GFX_PLATFORM,
}

impl igRegistry {
    pub fn new(_platform: IG_CORE_PLATFORM) -> Self {
        igRegistry {
            build_tool: BuildTool::None,
            platform: _platform.clone(),
            gfx_platform: IG_GFX_PLATFORM::from(_platform),
        }
    }
}
