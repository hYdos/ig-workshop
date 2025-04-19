use crate::client::archive::CArchive;
use crate::client::cdn::CContentDeployment;
use crate::client::precache::CPrecacheManager;
use crate::core::ig_registry::igRegistry;

/// Stores all structs related to the client implementation of Alchemy.
pub struct CClient {
    pub precache_manager: CPrecacheManager,
    /// Client implementation of archive loading. Used on iOS builds for the CDN system.
    pub archive_loader: CArchive,
    pub content_deployment: CContentDeployment
}

impl CClient {
    pub fn init(_ig_registry: &igRegistry) -> CClient {
        CClient {
            precache_manager: CPrecacheManager::new(),
            archive_loader: Default::default(),
            content_deployment: CContentDeployment { enabled: false },
        }
    }
}
