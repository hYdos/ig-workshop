#![allow(non_snake_case)]

use crate::core::ig_ark_core::{igArkCore, EGame};
use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_file_context::igFileContext;
use crate::core::ig_memory::igMemoryPool;
use crate::core::ig_objects::{igAny, ObjectExt};
use crate::core::ig_registry::igRegistry;
use crate::core::meta::ig_metadata_manager::{
    __internalObjectBase, igMetaObject, FieldDoesntExist, SetObjectFieldError,
};
use crate::util::ig_common::igAlchemy;
use std::any::Any;
use std::ops::Sub;
use std::sync::{Arc, RwLock};
use std::time::Instant;

fn load_alchemy() -> igAlchemy {
    let start_time = Instant::now();
    let ig_file_context = igFileContext::new("".to_string());
    let ig_registry = igRegistry::new(IG_CORE_PLATFORM::IG_CORE_PLATFORM_CAFE);
    let ig_alchemy = igAlchemy::new(
        ig_file_context,
        ig_registry,
        igArkCore::new(
            EGame::EV_SkylandersSuperchargers,
            IG_CORE_PLATFORM::IG_CORE_PLATFORM_CAFE,
        ),
    );
    let total_time = Instant::now().sub(start_time);
    println!("Alchemy loaded in {:?}", total_time);
    ig_alchemy
}

/// Verifies the metadata system is loading types correctly.
#[test]
fn test_metadata_system() {
    let mut ig_alchemy = load_alchemy();

    // Test: Test loading every single meta object
    ig_alchemy.ark_core.metadata_manager.load_all();
}

// Not a real type, mock up of the real igModelData
struct igModelData {
    pub _min: Vec<i32>,
    pub _max: Vec<i32>,
    pub _transforms: Vec<i32>,
    pub _transformHierarchy: Vec<i32>,
    pub _drawCalls: Vec<i32>,
    pub _drawCallTRansformIndices: Vec<i32>,
    pub _morphWeightTransforms: Vec<i32>,
    pub _blendMatrixIndices: Vec<i32>,
}

impl __internalObjectBase for igModelData {
    fn meta_type(&self) -> Arc<RwLock<igMetaObject>> {
        todo!()
    }

    fn internal_pool(&self) -> &igMemoryPool {
        todo!()
    }

    fn set_pool(&mut self, pool: igMemoryPool) {
        todo!()
    }

    fn set_field(
        &mut self,
        name: &str,
        value: Option<igAny>,
    ) -> Result<(), SetObjectFieldError> {
        todo!()
    }

    fn get_non_null_field(&self, name: &str) -> Result<igAny, FieldDoesntExist> {
        todo!()
    }

    fn get_field(
        &self,
        name: &str,
    ) -> Result<Option<Arc<RwLock<(dyn Any + Send + Sync + 'static)>>>, FieldDoesntExist> {
        todo!()
    }

    fn as_any(&self) -> &(dyn Any + Send + Sync) {
        todo!()
    }

    fn as_mut_any(&mut self) -> &mut (dyn Any + Send + Sync) {
        todo!()
    }
}

struct igModelInfo {
    pub _modelData: Arc<RwLock<igModelData>>,
}

impl __internalObjectBase for igModelInfo {
    fn meta_type(&self) -> Arc<RwLock<igMetaObject>> {
        todo!()
    }

    fn internal_pool(&self) -> &igMemoryPool {
        todo!()
    }

    fn set_pool(&mut self, pool: igMemoryPool) {
        todo!()
    }

    fn set_field(
        &mut self,
        name: &str,
        value: Option<igAny>,
    ) -> Result<(), SetObjectFieldError> {
        todo!()
    }

    fn get_non_null_field(&self, name: &str) -> Result<igAny, FieldDoesntExist> {
        todo!()
    }

    fn get_field(
        &self,
        name: &str,
    ) -> Result<Option<Arc<RwLock<(dyn Any + Send + Sync + 'static)>>>, FieldDoesntExist> {
        todo!()
    }

    fn as_any(&self) -> &(dyn Any + Send + Sync) {
        todo!()
    }

    fn as_mut_any(&mut self) -> &mut (dyn Any + Send + Sync) {
        todo!()
    }
}

/// Verifies the types can be worked with in a fluent way. Basically just a way to verify the intended syntax for using the metadata system is working correctly.
#[test]
fn test_type_usability() {
    let mut ig_alchemy = load_alchemy();
    let file_driver_moneybone = ig_alchemy
        .object_stream_manager
        .load(
            &mut ig_alchemy.file_context,
            &ig_alchemy.registry,
            &mut ig_alchemy.ark_core.metadata_manager,
            &mut ig_alchemy.ig_ext_ref_system,
            "DriverMoneybone".to_string(),
        )
        .unwrap();

    if let Ok(driver_moneybone) = file_driver_moneybone.read() {
        let models: Vec<Arc<RwLock<igModelInfo>>> = driver_moneybone
            .object_list
            .read()
            .unwrap()
            .iter()
            .filter_map(|x| x.clone().downcast::<igModelInfo>())
            .collect();

        println!("Model Count: {}", models.len());
        for model in models {
            if let Ok(model) = model.read().unwrap()._modelData.read() {
                println!("Model Draw Call Count: {}", model._drawCalls.len());
            }
        }
    };
}
