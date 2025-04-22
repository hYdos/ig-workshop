use crate::client::archive::CArchive;
use crate::client::cdn::CContentDeployment;
use crate::client::client::CClient;
use crate::core::external_ref::igExternalReferenceSystem;
use crate::core::ig_ark_core::EGame;
use crate::core::ig_file_context::{get_file_name, igFileContext};
use crate::core::ig_objects::igObjectStreamManager;
use crate::core::ig_registry::{igRegistry, BuildTool};
use crate::core::memory::EMemoryPoolID;
use crate::core::meta::ig_metadata_manager::igMetadataManager;
use crate::util::ig_common::{get_platform_string, igAlchemy};
use log::{error, info};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::Arc;
use strum::IntoEnumIterator;

pub trait CResourcePreCacher: Send + Sync {
    fn precache(&mut self);
    fn recache(&mut self);
    fn uncache(&mut self);
}

pub struct CPrecacheManager {
    pub resource_pre_cachers: Vec<Arc<dyn CResourcePreCacher>>,
    pub resource_pre_cacher_lookup: HashMap<String, Arc<dyn CResourcePreCacher>>,
    pub pool_package_lookup: HashMap<EMemoryPoolID, Vec<String>>,
}

pub struct COtherPackagePreCacher;
impl CResourcePreCacher for COtherPackagePreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CCharacterDataPreCacher;
impl CResourcePreCacher for CCharacterDataPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CSkinPreCacher;
impl CResourcePreCacher for CSkinPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CHavokAnimDBPreCacher;
impl CResourcePreCacher for CHavokAnimDBPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CHavokPhysicsSystemPreCacher;
impl CResourcePreCacher for CHavokPhysicsSystemPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CTexturePreCacher;
impl CResourcePreCacher for CTexturePreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CVfxPreCacher;
impl CResourcePreCacher for CVfxPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CShaderPreCacher;
impl CResourcePreCacher for CShaderPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CMotionPathPreCacher;
impl CResourcePreCacher for CMotionPathPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CIgFilePreCacher;
impl CResourcePreCacher for CIgFilePreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CMaterialPreCacher;
impl CResourcePreCacher for CMaterialPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CEntityPreCacher;
impl CResourcePreCacher for CEntityPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CGuiProjectPreCacher;
impl CResourcePreCacher for CGuiProjectPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CFontPreCacher;
impl CResourcePreCacher for CFontPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CLanguageFilePreCacher;
impl CResourcePreCacher for CLanguageFilePreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CModelPreCacher;
impl CResourcePreCacher for CModelPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CBehaviorPreCacher;
impl CResourcePreCacher for CBehaviorPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CBehaviorGraphDataPreCacher;
impl CResourcePreCacher for CBehaviorGraphDataPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CBehaviorEventPreCacher;
impl CResourcePreCacher for CBehaviorEventPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CBehaviorAssetPreCacher;
impl CResourcePreCacher for CBehaviorAssetPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CNavMeshPreCacher;
impl CResourcePreCacher for CNavMeshPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

pub struct CScriptPreCacher;
impl CResourcePreCacher for CScriptPreCacher {
    fn precache(&mut self) {
        todo!()
    }

    fn recache(&mut self) {
        todo!()
    }

    fn uncache(&mut self) {
        todo!()
    }
}

impl CPrecacheManager {
    pub fn new() -> CPrecacheManager {
        let mut pool_package_lookup = HashMap::with_capacity(EMemoryPoolID::MP_POOL_COUNT as usize);
        for pool in EMemoryPoolID::iter() {
            pool_package_lookup.insert(pool, Vec::<String>::new());
        }

        CPrecacheManager {
            resource_pre_cachers: Vec::with_capacity(0x24),
            resource_pre_cacher_lookup: HashMap::with_capacity(0x24),
            pool_package_lookup,
        }
    }

    pub fn register_pre_cachers(&mut self) {
        self.register("pkg", Arc::new(COtherPackagePreCacher));
        self.register("character_data", Arc::new(CCharacterDataPreCacher));
        self.register("actorskin", Arc::new(CSkinPreCacher));
        self.register("havokanimdb", Arc::new(CHavokAnimDBPreCacher));
        self.register("havokrigidbody", Arc::new(CHavokPhysicsSystemPreCacher));
        self.register("havokphysicssystem", Arc::new(CHavokPhysicsSystemPreCacher));
        self.register("texture", Arc::new(CTexturePreCacher));
        self.register("effect", Arc::new(CVfxPreCacher));
        self.register("shader", Arc::new(CShaderPreCacher));
        self.register("motionpath", Arc::new(CMotionPathPreCacher));
        self.register("igx_file", Arc::new(CIgFilePreCacher));
        self.register("material_instances", Arc::new(CMaterialPreCacher));
        self.register("igx_entities", Arc::new(CEntityPreCacher));
        self.register("gui_project", Arc::new(CGuiProjectPreCacher));
        self.register("font", Arc::new(CFontPreCacher));
        self.register("lang_file", Arc::new(CLanguageFilePreCacher));
        self.register("spawnmesh", Arc::new(CIgFilePreCacher));
        self.register("model", Arc::new(CModelPreCacher));
        self.register("sky_model", Arc::new(CModelPreCacher));
        self.register("behavior", Arc::new(CBehaviorPreCacher));
        self.register("graphdata_behavior", Arc::new(CBehaviorGraphDataPreCacher));
        self.register("events_behavior", Arc::new(CBehaviorEventPreCacher));
        self.register("asset_behavior", Arc::new(CBehaviorAssetPreCacher));
        self.register("hkb_behavior", Arc::new(CBehaviorAssetPreCacher));
        self.register("hkc_character", Arc::new(CBehaviorAssetPreCacher));
        self.register("navmesh", Arc::new(CNavMeshPreCacher));
        self.register("script", Arc::new(CScriptPreCacher));

        // _packagesPerPool = new igVector<igVector<string>>();
        // _packagesPerPool.SetCapacity((int)EMemoryPoolID.MP_MAX_POOL);
        // mObjectDirectoryLists = new igVector<igObjectDirectoryList>();
        // mObjectDirectoryLists.SetCapacity((int)EMemoryPoolID.MP_MAX_POOL);
        // for(int i = 0; i < (int)EMemoryPoolID.MP_MAX_POOL; i++)
        // {
        //     _packagesPerPool.Append(new igVector<string>());
        //     mObjectDirectoryLists.Append(new igObjectDirectoryList());
        // }
    }

    fn register(&mut self, name: &str, resource_pre_cacher: Arc<dyn CResourcePreCacher>) {
        self.resource_pre_cachers.push(resource_pre_cacher.clone());
        self.resource_pre_cacher_lookup
            .insert(name.to_string(), resource_pre_cacher);
    }

    pub fn precache_package(
        &self,
        archive_loader: &CArchive,
        cdn: &CContentDeployment,
        ig_registry: &igRegistry,
        ig_file_context: &igFileContext,
        ig_object_stream_manager: &mut igObjectStreamManager,
        ig_ext_ref_system: &mut igExternalReferenceSystem,
        ig_metadata_manager: &mut igMetadataManager,
        package_name: String,
        pool_id: EMemoryPoolID,
    ) {
        let mut package_path = package_name.to_lowercase();

        if !package_path.starts_with("packages") {
            package_path = format!("packages/{}", package_path);
        }

        if !package_path.ends_with("_pkg.igz") {
            package_path = format!("{}_pkg.igz", package_path);
        }

        if self.package_cached(&package_path, pool_id) {
            return;
        }

        // igCauldron removed the extension here however it never has one so ???
        archive_loader
            .open(
                cdn,
                ig_file_context,
                ig_registry,
                get_file_name(&package_path.trim_end_matches("_pkg.igz")).unwrap(),
                0,
            )
            .unwrap();

        let _pkg_dir = ig_object_stream_manager
            .load(
                ig_file_context,
                ig_registry,
                ig_metadata_manager,
                ig_ext_ref_system,
                package_path,
            )
            .unwrap();
    }

    pub fn package_cached(&self, package_name: &str, pool_id: EMemoryPoolID) -> bool {
        let pool_packages = &self.pool_package_lookup[&pool_id];
        pool_packages.contains(&package_name.to_lowercase())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum LoaderTask {
    Unknown,
    LooseIga,
    FullPackage,
    LoosePackage,
    EngineType,
    NoOp,
}

pub static ENV_LOOKUP: Lazy<
    HashMap<&'static str, Box<dyn Fn(&igRegistry) -> String + Send + Sync>>,
> = Lazy::new(|| {
    let mut m = HashMap::with_capacity(1);
    m.insert(
        "platform_string",
        Box::new(|reg: &igRegistry| get_platform_string(reg.platform.clone()))
            as Box<dyn Fn(&igRegistry) -> String + Send + Sync>,
    );
    m
});

pub fn load_init_script(game: EGame, is_weakly_loaded: bool, ig_alchemy: &mut igAlchemy) {
    let script_path = PathBuf::from(format!("ArkCore/{:?}/initscript", game));
    let init_script = File::open(script_path).expect("initscript not found");
    let reader = BufReader::new(init_script);
    let file_lines = reader.lines();

    let mut task = LoaderTask::LooseIga;
    let mut line_number = 0;
    for raw_line in file_lines {
        let line = raw_line.unwrap();
        line_number += 1;
        if line.is_empty() {
            continue;
        }

        if line.chars().nth(0) == Some('[') {
            if line.chars().nth(line.len() - 1) != Some(']') {
                error!(
                    "Invalid initscript. Unterminated '[' on line {}",
                    line_number
                );
                break;
            }

            task = parse_task(line, is_weakly_loaded);

            if task == LoaderTask::Unknown {
                error!(
                    "Invalid initscript. Unknown task type on line {}",
                    line_number
                );
            }
        } else {
            let path = parse_file_path(line, &ig_alchemy.registry);
            if path.is_none() {
                error!(
                    "Invalid initscript. Malformed filepath on line {}",
                    line_number
                );
            }

            process_task(
                &mut ig_alchemy.client,
                &mut ig_alchemy.file_context,
                &mut ig_alchemy.registry,
                &mut ig_alchemy.object_stream_manager,
                &mut ig_alchemy.ig_ext_ref_system,
                &mut ig_alchemy.ark_core.metadata_manager,
                task.clone(),
                path.unwrap(),
            );
        }
    }
    info!("initscript -> done");
}

fn parse_task(line: String, is_weakly_loaded: bool) -> LoaderTask {
    let task_name = &line[1..line.len() - 1];
    match task_name {
        "loose_pak" => LoaderTask::LoosePackage,
        "loose_pak_lab" => LoaderTask::LooseIga,
        "full_package_lab" => {
            if is_weakly_loaded {
                LoaderTask::NoOp
            } else {
                LoaderTask::FullPackage
            }
        }
        "engine_type" => LoaderTask::EngineType,
        _ => LoaderTask::Unknown,
    }
}

pub fn parse_file_path(line: String, ig_registry: &igRegistry) -> Option<String> {
    let mut processed = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '{' {
                    chars.next();

                    let mut token = String::new();
                    let mut found_closing = false;
                    while let Some(ch) = chars.next() {
                        if ch == '}' {
                            found_closing = true;
                            break;
                        }
                        token.push(ch);
                    }

                    if !found_closing {
                        return None;
                    }

                    let executor = ENV_LOOKUP.get(token.as_str())?;
                    processed.push_str(&executor(ig_registry));
                    continue;
                } else {
                    return None;
                }
            }
        } else {
            processed.push(c);
        }
    }

    Some(processed)
}

fn process_task(
    client: &mut CClient,
    ig_file_context: &mut igFileContext,
    ig_registry: &mut igRegistry,
    ig_object_stream_manager: &mut igObjectStreamManager,
    ig_ext_ref_system: &mut igExternalReferenceSystem,
    ig_metadata_manager: &mut igMetadataManager,
    task: LoaderTask,
    line: String,
) {
    info!("initscript -> {:?} {}", task, line);
    let precache_manager = &client.precache_manager;

    match task {
        LoaderTask::LooseIga => {
            ig_file_context.load_archive(ig_registry, line);
        }
        LoaderTask::FullPackage => {
            precache_manager.precache_package(
                &mut client.archive_loader,
                &client.content_deployment,
                ig_registry,
                ig_file_context,
                ig_object_stream_manager,
                ig_ext_ref_system,
                ig_metadata_manager,
                line,
                EMemoryPoolID::MP_DEFAULT,
            );
        }
        LoaderTask::LoosePackage => {
            ig_file_context.load_archive(ig_registry, format!("app:/archives/{}.pak", line));
        }
        LoaderTask::EngineType => match line.as_str() {
            "None" => ig_registry.build_tool = BuildTool::None,
            "AlchemyLaboratory" => ig_registry.build_tool = BuildTool::AlchemyLaboratory,
            "TfbTool" => ig_registry.build_tool = BuildTool::TfbTool,
            _ => {
                error!("Invalid initscript. {} is not a valid EngineType", line);
            }
        },
        LoaderTask::NoOp | LoaderTask::Unknown => {}
    }
}
