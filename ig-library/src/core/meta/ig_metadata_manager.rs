use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_custom::{igDataList, igNameList, igObjectList};
use crate::core::ig_fs::Endian;
use crate::core::ig_memory::igMemoryPool;
use crate::core::ig_objects::{igAny, ObjectExt};
use crate::core::load::ig_igz_loader::IgzLoaderContext;
use crate::core::meta::field::ig_metafield_registry::igMetafieldRegistry;
use crate::core::meta::ig_xml_metadata::{ArcMetaEnum, ArcMetaField, ArkMetaObjectField, MetaObject, RawArkMetaObjectField};
use log::{debug, error, info};
use phf::phf_map;
use std::any::Any;
use std::collections::HashMap;
use std::io::Cursor;
use std::ops::Sub;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::time::Instant;
use std::todo;
use strum_macros::Display;

type MetaObjectConstructor = fn(
    ig_meta_object: Arc<igMetaObject>,
    pool: igMemoryPool,
) -> Result<Arc<RwLock<dyn __internalObjectBase>>, igMetaInstantiationError>;

/// Stores every registered metaobject implementation that can be constructed.
pub static TYPE_TO_METAOBJECT_LOOKUP: phf::Map<
    &str,
    MetaObjectConstructor,
> = phf_map! {
    "igObjectList"            => igObjectList::construct,
    "igStringRefList"            => igDataList::<Arc<str>>::construct,
    "igNameList"            => igNameList::construct,
};

/// Fast structure used to manage and create new instances of metaobjects, metafields, and metaenums
pub struct igMetadataManager {
    meta_fields: HashMap<Arc<str>, ArcMetaField>,
    meta_enums: HashMap<Arc<str>, ArcMetaEnum>,
    meta_objects: HashMap<Arc<str>, MetaObject>,
    object_meta_lookup: HashMap<Arc<str>, Arc<igMetaObject>>,
    /// The platform the metadata system is targeting. Can be stored here because we know this is not used between different loaded games.
    platform: IG_CORE_PLATFORM,
    pub meta_field_registry: igMetafieldRegistry,
}

impl igMetadataManager {
    /// Takes in igz context and sets the fields of the passed in ig_object. called from ig_igz_loader.
    pub(crate) fn read_igz_fields(
        &self,
        handle: &mut Cursor<Vec<u8>>,
        endian: Endian,
        ctx: &IgzLoaderContext,
        ig_object: Arc<RwLock<dyn __internalObjectBase>>,
    ) {
        let object_offset = handle.position();
        let meta = ig_object.read().unwrap().meta_type();
        debug!("igObject(name={}) fields are being set", meta.name);
        let fields = &meta.field_storage.name_lookup;

        for (name, field) in fields {
            match name.as_ref() {
                "igStaticMetaField" | "igPropertyFieldMetaField" => {
                    // ignored, not important on a per-object basis.
                }
                &_ => {
                    debug!("Setting up igz field(name={}, type={})", name, field._type);
                    handle.set_position(object_offset + field.offset as u64);
                    let metafield = self.meta_field_registry.get(field.clone(), self, self.platform.clone());
                    let value = metafield.value_from_igz(handle, &endian, ctx, &self.meta_field_registry, &self);
                    if let Ok(mut guard) = ig_object.write() {
                        match guard.set_field(name.as_ref(), value) {
                            Ok(_) => {}
                            Err(SetObjectFieldError::FieldDoesntExist) => {
                                error!("When reading the igz value for the field {}, got SetObjectFieldError::FieldDoesntExist", name);
                                panic!("Alchemy Error! Check the logs.")
                            }
                            Err(SetObjectFieldError::InvalidValueType) => {
                                error!("When reading the igz value for the field {}, got SetObjectFieldError::InvalidValueType", name);
                                panic!("Alchemy Error! Check the logs.")
                            }
                            Err(SetObjectFieldError::InvalidValue) => {
                                error!("When reading the igz value for the field {}, got SetObjectFieldError::InvalidValue", name);
                                panic!("Alchemy Error! Check the logs.")
                            }
                            Err(SetObjectFieldError::Unknown) => {
                                error!("When reading the igz value for the field {}, got SetObjectFieldError::Unknown", name);
                                panic!("Alchemy Error! Check the logs.")
                            }
                        }
                    }
                }
            }
        }
    }
}

impl igMetadataManager {
    /// Loads every single [igMetaObject] possible from the meta object's that have been deserialized from metaobjects.xml. This method has use in scenarios where loading all types ahead of time for testing, runtime (such as igPlayer) applications, or debugging could benefit from not waiting in the middle of their application
    pub fn load_all(&mut self) {
        let start_time = Instant::now();
        let type_names: Vec<String> = self
            .meta_objects
            .keys()
            .map(|x1| x1.to_string()) // Become owners of the strings to stop borrowing from ourselves
            .collect();

        for _type in type_names {
            let _meta = self.get_or_create_meta(&_type).unwrap();
        }

        let total_time = Instant::now().sub(start_time);
        info!("igMetaObject's loaded and cached in {:?}", total_time);
    }
}

/// Represents different error states that can be achieved from calling [__internalObjectBase::set_field]
#[derive(Debug)]
pub enum SetObjectFieldError {
    /// Returned when the type of the value passed in to the function is invalid for the meta field stored.
    InvalidValueType,
    /// Returned when the type is correct but there is an issue with the value passed in.
    InvalidValue,
    /// Returned when the field you are trying to set does not exist.
    FieldDoesntExist,
    /// Returned when none of the other error conditions are met.
    Unknown,
}

/// Only possible error generatable from [__internalObjectBase::get_field]
pub struct FieldDoesntExist;

/// Represents an object that can be converted from igz or other data into a igObject
pub trait __internalObjectBase: Sync + Send {
    /// Returns the [igMetaObject] that built the object
    fn meta_type(&self) -> Arc<igMetaObject>;
    /// Returns a reference to the [igMemoryPool] used to construct the object
    fn internal_pool(&self) -> &igMemoryPool;
    /// Changes the target [igMemoryPool] for reading or writing the object
    fn set_pool(&mut self, pool: igMemoryPool);
    /// Sets the value in the object with the name specified and value
    fn set_field(
        &mut self,
        name: &str,
        value: Option<igAny>,
    ) -> Result<(), SetObjectFieldError>;
    fn get_non_null_field(
        &self,
        name: &str,
    ) -> Result<igAny, FieldDoesntExist>;
    
    fn get_field(
        &self,
        name: &str,
    ) -> Result<Option<igAny>, FieldDoesntExist>;
    fn as_any(&self) -> &(dyn Any + Send + Sync);
    fn as_mut_any(&mut self) -> &mut (dyn Any + Send + Sync);
}

/// Represents an object with no programmer-made translation. However, programmer translated (structs implementing __internalObjectBase) may use this struct in order to build their representation of an igObject. This has not been implemented though, and will most likely change in the future.
// TODO: this class should NEVER be used. It WILL cause issues if there are two tools and one tool expects a class while the other one doesn't have one. I can't think of too many uses for it anyways. Maybe remove it in the future?
#[deprecated]
pub struct igGenericObject {
    meta: Arc<igMetaObject>,
    constructed_field_storage: Vec<RwLock<igConstructedField>>,
    internal_pool: igMemoryPool,
}

impl igGenericObject {
    fn new(
        meta: Arc<igMetaObject>,
        _pool: igMemoryPool,
    ) -> Result<Arc<RwLock<dyn __internalObjectBase>>, igMetaInstantiationError> {
        let mut constructed_field_storage = Vec::new();
        for entry in &meta.field_storage.name_lookup {
            constructed_field_storage.push(RwLock::new(igConstructedField {
                name: entry.0.clone(),
                value: None,
            }))
        }

        Ok(Arc::new(RwLock::new(igGenericObject {
            meta: meta.clone(),
            constructed_field_storage,
            internal_pool: _pool,
        })))
    }
}

impl __internalObjectBase for igGenericObject {
    fn meta_type(&self) -> Arc<igMetaObject> {
        self.meta.clone()
    }

    fn internal_pool(&self) -> &igMemoryPool {
        &self.internal_pool
    }

    fn set_pool(&mut self, pool: igMemoryPool) {
        self.internal_pool = pool;
    }

    fn set_field(
        &mut self,
        name: &str,
        value: Option<igAny>,
    ) -> Result<(), SetObjectFieldError> {
        for field in &self.constructed_field_storage {
            if field.read().unwrap().name.as_ref() == name {
                field.write().unwrap().value = value;
                return Ok(());
            }
        }

        Err(SetObjectFieldError::FieldDoesntExist)
    }

    fn get_non_null_field(
        &self,
        name: &str,
    ) -> Result<igAny, FieldDoesntExist> {
        Ok(self
            .get_field(name)?
            .expect("called get_non_null_field on a null value"))
    }

    fn get_field(
        &self,
        name: &str,
    ) -> Result<Option<Arc<RwLock<(dyn Any + Send + Sync + 'static)>>>, FieldDoesntExist> {
        for field in &self.constructed_field_storage {
            if field.read().unwrap().name.as_ref() == name {
                return Ok(field.read().unwrap().value.clone());
            }
        }

        Err(FieldDoesntExist)
    }

    fn as_any(&self) -> &(dyn Any + Send + Sync) {
        self
    }

    fn as_mut_any(&mut self) -> &mut (dyn Any + Send + Sync) {
        self
    }
}

/// I don't have much to say on this one. Please check impl functions for usages you may like. Any igObject you want to use should probably not let you get this deep. This is getting deep into the metadata system. Please implement types you use!
pub struct igConstructedField {
    /// The name of the constructed field
    pub name: Arc<str>,
    /// Represents the currently stored value. Wrapped in [Arc] to work with how Alchemy is designed to work and [RwLock] for mutability and thread safety
    pub value: Option<igAny>,
}

/// Information related to the construction of new field instances.
pub struct igMetaFieldInfo {
    pub ark_info: ArkMetaObjectField,
    pub _type: Arc<str>,
    pub name: Option<Arc<str>>,
    pub size: u32,
    pub offset: u16,
}

/// Type designed for ergonomics and to keep speed up
struct FieldStorage {
    /// All field will be present in this map. Use the offset of a field to look it up
    offset_lookup: HashMap<u16, Arc<igMetaFieldInfo>>,
    /// NOT all field will be present in this map. Any field not using a name will not be present
    name_lookup: HashMap<Arc<str>, Arc<igMetaFieldInfo>>,
}

impl FieldStorage {
    pub fn new(fields: Vec<Arc<igMetaFieldInfo>>) -> FieldStorage {
        let mut offset_lookup = HashMap::new();
        let mut name_lookup = HashMap::new();

        for x in fields {
            offset_lookup.insert(x.offset, x.clone());
            if let Some(name) = &x.name {
                name_lookup.insert(name.clone(), x.clone());
            }
        }

        FieldStorage {
            offset_lookup,
            name_lookup,
        }
    }
}

type InternalMetaObjectConstructor = fn(
    Arc<igMetaObject>,
    igMemoryPool,
) -> Result<Arc<RwLock<dyn __internalObjectBase>>, igMetaInstantiationError>;

/// Represents the data needed to instantiate an instance of the meta object stored.
pub struct igMetaObject {
    pub name: Arc<str>,
    constructor: InternalMetaObjectConstructor,
    #[allow(dead_code)] // FIXME: the metadata system (at some point in the future) needs to fall back on a parent implementation instead of igGenericObject if possible.
    parent: Option<Arc<igMetaObject>>,
    field_storage: FieldStorage,
}

/// Describes all possible errors returned from the function [igMetaObject::instantiate]
#[derive(Debug, Display)]
pub enum igMetaInstantiationError {
    /// Returned when the construction succeeded but the default field failed to be set.
    SetupDefaultFieldsError,
    /// Returned when the construction succeeded but the type constructed does not match the expected return type.
    TypeMismatchError(Arc<str>),
}

impl igMetaObject {
    /// Creates a new instance of [Arc<RwLock<T>>] on success. On failure [igMetaInstantiationError] will be returned. [T] is expected ot match the type associated with the [igMetaObject] provided. If there is no registered type for the metadata, [igGenericObject] will be constructed
    pub fn instantiate<T>(
        self: Arc<igMetaObject>,
        _source_pool: igMemoryPool,
        _set_fields: bool,
    ) -> Result<Arc<RwLock<T>>, igMetaInstantiationError>
    where
        T: __internalObjectBase + 'static,
    {
        let fun = self.constructor;
        let arc = fun(self, _source_pool)?;
        let _type = match arc.clone().downcast::<T>() {
            Some(t) => Ok(t),
            None => Err(igMetaInstantiationError::TypeMismatchError(
                arc.read().unwrap().meta_type().name.clone(),
            )),
        }?;

        // This will always succeed. We just created the type
        let guard = _type.write().unwrap();
        if _set_fields {
            todo!()
        }
        drop(guard);

        Ok(_type)
    }

    pub fn raw_instantiate(
        self: Arc<igMetaObject>,
        _source_pool: igMemoryPool,
        _set_fields: bool,
    ) -> Result<Arc<RwLock<dyn __internalObjectBase>>, igMetaInstantiationError> {
        let fun = self.constructor;
        let arc = fun(self, _source_pool)?;
        Ok(arc)
    }
}

#[derive(Debug)]
pub enum MetaInitializationFailedException {} 

impl igMetadataManager {
    /// Will search the cache for the type from the given name, if there is no match, It will load the type now and cache it for later use
    pub fn get_or_create_meta(&mut self, type_name: &str) -> Result<Arc<igMetaObject>, MetaInitializationFailedException> {
        if self.object_meta_lookup.contains_key(type_name) {
            return Ok(self.object_meta_lookup[type_name].clone());
        }

        let ig_object_meta = Arc::new(self.create_object_meta(type_name));
        self.object_meta_lookup
            .insert(Arc::from(type_name), ig_object_meta.clone());
        Ok(ig_object_meta)
    }

    fn create_object_meta(&self, type_name: &str) -> igMetaObject {
        let object = &self.meta_objects[type_name];
        let mut parent_meta = None;

        if let Some(parent) = &object.base_type {
            parent_meta = Some(Arc::new(self.create_object_meta(parent)));
        }

        let field_storage = self.get_current_fields(&self.platform, parent_meta.clone(), object);

        if let Some(constructor) = TYPE_TO_METAOBJECT_LOOKUP.get(type_name) {
            igMetaObject {
                name: Arc::from(type_name),
                constructor: *constructor,
                parent: parent_meta,
                field_storage,
            }
        } else {
            igMetaObject {
                name: Arc::from(type_name),
                constructor: igGenericObject::new,
                parent: parent_meta,
                field_storage,
            }
        }
    }

    pub(crate) fn calculate_size(&self, object: &RawArkMetaObjectField, platform: &IG_CORE_PLATFORM) -> u32 {
        self.meta_fields[&object._type].platform_info[platform].size as u32
    }

    /// Loops through all available field and builds up a list of field for the current meta object taking into account overriden field
    fn get_current_fields(
        &self,
        platform: &IG_CORE_PLATFORM,
        parent: Option<Arc<igMetaObject>>,
        current_object: &MetaObject,
    ) -> FieldStorage {
        // TODO: handle however compound field work
        if let Some(parent) = &parent {
            let parent_fields = &parent.clone().field_storage.offset_lookup;
            let mut new_fields: Vec<Arc<igMetaFieldInfo>> = Vec::new();

            for parent_field in parent_fields {
                let mut overriden = false;

                for override_field in &current_object.overriden_fields {
                    let override_field = override_field.read().unwrap();
                    if *parent_field.0 == override_field.offset {
                        new_fields.push(Arc::new(igMetaFieldInfo {
                            ark_info: Arc::new(RwLock::new(override_field.clone())),
                            _type: override_field.clone()._type,
                            name: override_field.clone().name,
                            size: igMetadataManager::calculate_size(
                                self,
                                &override_field,
                                platform,
                            ),
                            offset: override_field.offset,
                        }));
                        overriden = true;
                        break;
                    }
                }

                if !overriden {
                    new_fields.push(parent_field.1.clone())
                }
            }

            for field in &current_object.new_fields {
                let field = field.read().unwrap();

                new_fields.push(Arc::new(igMetaFieldInfo {
                    ark_info: Arc::new(RwLock::new(field.clone())),
                    _type: field.clone()._type,
                    name: field.clone().name,
                    size: igMetadataManager::calculate_size(self, &field, platform),
                    offset: field.offset,
                }));
            }

            FieldStorage::new(new_fields)
        } else {
            let mut offset_lookup: HashMap<u16, Arc<igMetaFieldInfo>> = HashMap::new();
            let mut name_lookup: HashMap<Arc<str>, Arc<igMetaFieldInfo>> = HashMap::new();

            for field in &current_object.new_fields {
                let lock = field.read().unwrap();

                let new_field = Arc::new(igMetaFieldInfo {
                    ark_info: Arc::new(RwLock::new(lock.clone())),
                    _type: lock._type.clone(),
                    name: lock.name.clone(),
                    size: self.calculate_size(&lock, platform),
                    offset: lock.offset,
                });

                offset_lookup.insert(new_field.offset, new_field.clone());
                if let Some(name) = &lock.name {
                    name_lookup.insert(name.clone(), new_field.clone());
                }
            }

            FieldStorage {
                offset_lookup,
                name_lookup,
            }
        }
    }

    /// Initializes a new [igMetadataManager]. Types here are converted into maps early in order to save on lookup cost later
    pub fn new(
        field_list: Vec<ArcMetaField>,
        enum_list: Vec<ArcMetaEnum>,
        object_list: Vec<MetaObject>,
        platform: IG_CORE_PLATFORM,
    ) -> igMetadataManager {
        let mut meta_fields = HashMap::with_capacity(field_list.len());
        for field in field_list {
            meta_fields.insert(field.name.clone(), field);
        }

        let mut meta_enums = HashMap::with_capacity(enum_list.len());
        for enm in enum_list {
            meta_enums.insert(enm.ref_name.clone(), enm);
        }

        let mut meta_objects = HashMap::with_capacity(object_list.len());
        for obj in object_list {
            meta_objects.insert(obj.ref_name.clone(), obj);
        }

        igMetadataManager {
            object_meta_lookup: HashMap::with_capacity(meta_objects.len()),
            meta_fields,
            meta_enums,
            meta_objects,
            platform,
            meta_field_registry: igMetafieldRegistry::new(),
        }
    }

    pub fn get_enum<T: MetaEnumImpl>(&self, value_index: usize) -> T {
        let value = &self.meta_enums[T::META_KEY].values[value_index];
        if let Ok(return_value) = T::from_str(&value.name) {
            return_value
        } else {
            error!(
                "tried to get enum {} but conversion function has no match for {}",
                T::META_KEY,
                value.name
            );
            panic!("Alchemy Error! Check the logs.");
        }
    }
}

/// If you want to use an enum from the metadata inside your code, You need to implement this trait. It gives the metadata system some extra information about the enum's name in order to help it find what you are looking for
pub trait MetaEnumImpl: FromStr {
    /// The name of the enum you are from the metadata. For example: "IG_CORE_PLATFORM". See metaenums.xml for more enum's to choose from.
    const META_KEY: &'static str;
}
