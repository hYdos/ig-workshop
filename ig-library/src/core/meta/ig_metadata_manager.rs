use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_memory::igMemoryPool;
use crate::core::meta::ig_metafield::{igBoolMetaField, MetaFieldImpl};
use crate::core::meta::ig_xml_metadata::{MetaEnum, MetaField, MetaObject, RawMetaObjectField};
use log::{error, info};
use phf::phf_map;
use std::any::{type_name_of_val, Any};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Sub;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::time::Instant;
use strum_macros::Display;

/// Stores every registered meta field implementation available.
static TYPE_TO_METAFIELD_LOOKUP: phf::Map<&str, fn() -> Box<dyn MetaFieldImpl>> = phf_map! {
    "igBoolMetaField"            => || {Box::new(igBoolMetaField::new())}
};

/// Fast structure used to manage and create new instances of metaobjects, metafields, and metaenums
pub struct igMetadataManager {
    meta_fields: HashMap<Arc<str>, MetaField>,
    meta_enums: HashMap<Arc<str>, MetaEnum>,
    meta_objects: HashMap<Arc<str>, MetaObject>,
    object_meta_lookup: HashMap<Arc<str>, Arc<igMetaObject>>,
    /// The platform the metadata system is targeting. Can be stored here because we know this is not used between different loaded games.
    platform: IG_CORE_PLATFORM,
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
pub enum SetObjectFieldError {
    /// Returned when the type of the value passed in to the function is invalid for the meta field stored.
    InvalidValueType,
    /// Returned when the type is correct but there is an issue with the value passed in.
    InvalidValue,
    /// Returned when the field you are trying to set does not exist.
    MissingField,
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
    fn set_field(&mut self, name: &str, value: &dyn Any) -> Result<(), SetObjectFieldError>;
    fn get_field(&self, name: &str) -> Result<&dyn Any, FieldDoesntExist>;

    // Related to internal workings of metadata system. avoid if possible
    #[doc(hidden)]
    /// Allows mutating the type we think we are. Useful for constructing new objects and swapping to the real type
    fn as_any(&self) -> &dyn Any;
    #[doc(hidden)]
    /// No known use for this method yet, but is here as a placeholder for the future
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

/// Represents an object with no programmer made translation. However, programmer translated (structs implementing __internalObjectBase) may use this struct in order to build their representation of an igObject.
pub struct igGenericObject {
    meta: Arc<igMetaObject>,
    constructed_field_storage: Vec<RwLock<igConstructedField>>,
    internal_pool: igMemoryPool,
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

    fn set_field(&mut self, name: &str, value: &dyn Any) -> Result<(), SetObjectFieldError> {
        todo!()
    }

    fn get_field(&self, name: &str) -> Result<&dyn Any, FieldDoesntExist> {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// I don't have much to say on this one. Please check impl functions for usages you may like. Any igObject you want to use should probably not let you get this deep. This is getting deep into the metadata system. Please implement types you use!
pub struct igConstructedField {
    /// The name of the constructed field
    pub name: Arc<str>,
    /// Internal to the metadata system. This field stores the value of the
    #[doc(hidden)]
    pub(crate) metafield: Box<dyn MetaFieldImpl>,
}

impl Display for igConstructedField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let current_value = &self.metafield;
        write!(
            f,
            "{} (type: {})",
            self.name.as_ref(),
            type_name_of_val(&current_value.default_value())
        )
    }
}

// igCauldron cares about the platform here, we don't. Metadata cannot be shared between platforms and expected to work anyway.
#[derive(Debug)]
struct StoredField {
    name: Option<Arc<str>>,
    size: u32,
    offset: u16,
}

/// Type designed for ergonomics and to keep speed up
#[derive(Debug)]
struct FieldStorage {
    /// All fields will be present in this map. Use the offset of a field to look it up
    offset_lookup: HashMap<u16, Arc<StoredField>>,
    /// NOT all fields will be present in this map. Any field not using a name will not be present
    name_lookup: HashMap<Arc<str>, Arc<StoredField>>,
}

impl FieldStorage {
    pub fn new(fields: Vec<Arc<StoredField>>) -> FieldStorage {
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

/// Represents the data needed to instantiate an instance of the meta object stored.
#[derive(Debug)]
pub struct igMetaObject {
    pub name: Arc<str>,
    constructor: fn(&igMetaObject) -> Result<Arc<dyn Any + Send + Sync>, igMetaInstantiationError>,
    parent: Option<Arc<igMetaObject>>,
    field_storage: FieldStorage,
}

/// Describes all possible errors returned from the function [igMetaObject::instantiate]
#[derive(Debug, Display)]
pub enum igMetaInstantiationError {
    /// Returned when the construction succeeded but the default fields failed to be set.
    SetupDefaultFieldsError,
    /// Returned when the construction succeeded but the type constructed does not match the expected return type.
    TypeMismatchError,
}

impl igMetaObject {
    /// Creates a new instance of [Arc<RwLock<T>>] on success. On failure [igMetaInstantiationError] will be returned. [T] is expected ot match the type associated with the [igMetaObject] provided. If there is no registered type for the metadata, [igGenericObject] will be constructed
    pub fn instantiate<T>(
        &self,
        _source_pool: igMemoryPool,
        _set_fields: bool,
    ) -> Result<Arc<RwLock<T>>, igMetaInstantiationError>
    where
        T: __internalObjectBase + 'static,
    {
        let fun = self.constructor;
        let arc = fun(self)?;
        match Arc::downcast::<RwLock<T>>(arc) {
            Ok(_) | Err(_) => todo!(),
        }
    }
}

impl igMetadataManager {
    /// Will search the cache for the type from the given name, if there is no match, It will load the type now and cache it for later use
    pub fn get_or_create_meta(&mut self, type_name: &str) -> Result<Arc<igMetaObject>, ()> {
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

        igMetaObject {
            name: Arc::from(type_name),
            constructor: |_x| todo!(),
            parent: parent_meta,
            field_storage,
        }
    }

    fn calculate_size(&self, object: &RawMetaObjectField, platform: &IG_CORE_PLATFORM) -> u32 {
        self.meta_fields[&object._type].platform_info[platform].size as u32
    }

    /// Loops through all available fields and builds up a list of fields for the current meta object taking into account overriden fields
    fn get_current_fields(
        &self,
        platform: &IG_CORE_PLATFORM,
        parent: Option<Arc<igMetaObject>>,
        current_object: &MetaObject,
    ) -> FieldStorage {
        // TODO: handle however compound fields work
        if let Some(parent) = &parent {
            let parent_fields = &parent.clone().field_storage.offset_lookup;
            let mut new_fields: Vec<Arc<StoredField>> = Vec::new();

            for parent_field in parent_fields {
                let mut overriden = false;

                for override_field in &current_object.overriden_fields {
                    let override_field = override_field.read().unwrap();
                    if *parent_field.0 == override_field.offset {
                        new_fields.push(Arc::new(StoredField {
                            name: override_field.clone().name,
                            size: igMetadataManager::calculate_size(
                                &self,
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

                new_fields.push(Arc::new(StoredField {
                    name: field.clone().name,
                    size: igMetadataManager::calculate_size(&self, &field, platform),
                    offset: field.offset,
                }));
            }

            FieldStorage::new(new_fields)
        } else {
            let mut offset_lookup: HashMap<u16, Arc<StoredField>> = HashMap::new();
            let mut name_lookup: HashMap<Arc<str>, Arc<StoredField>> = HashMap::new();

            for field in &current_object.new_fields {
                let lock = field.read().unwrap();

                let new_field = Arc::new(StoredField {
                    name: lock.name.clone(),
                    size: igMetadataManager::calculate_size(&self, &lock, platform),
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
        field_list: Vec<MetaField>,
        enum_list: Vec<MetaEnum>,
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
