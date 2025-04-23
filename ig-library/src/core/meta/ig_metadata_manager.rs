use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::meta::ig_metafield::MetaFieldImpl;
use crate::core::meta::ig_xml_metadata::{MetaEnum, MetaField, MetaObject, RawMetaObjectField};
use log::{error, info};
use std::any::{type_name_of_val, Any};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Sub;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::time::Instant;

/// Fast structure used to manage and create new instances of metaobjects, metafields, and metaenums
pub struct igMetadataManager {
    meta_fields: HashMap<Arc<str>, MetaField>,
    meta_enums: HashMap<Arc<str>, MetaEnum>,
    meta_objects: HashMap<Arc<str>, MetaObject>,
    object_meta_lookup: HashMap<Arc<str>, Arc<igMetaObject>>,
    platform: IG_CORE_PLATFORM,
}

/// Missing implementation. Plan is for this to exist for every metaobject to allow conversion to/from nice types (example: igFloatMetaField -> f32)
trait igMetaFieldTranslator {
    fn read_igz_field();
    fn write_igz_field();
    fn get_size();
    fn get_alignment();
}

impl igMetadataManager {
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

/// Represents an object that can be converted from igz or other data into a igObject
pub trait __internalObjectBase: Sync + Send {
    /// Returns the type of the object instantiated
    fn meta_type(&self) -> Arc<str>;

    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl __internalObjectBase for igGenericObject {
    fn meta_type(&self) -> Arc<str> {
        self.name.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// Represents an object with no programmer made translation. However, programmer translated (structs implementing __internalObjectBase) may use this struct in order to build their representation of an igObject.
pub struct igGenericObject {
    pub name: Arc<str>,
    pub meta: Arc<igMetaObject>,
    pub constructed_field_storage: Vec<RwLock<igConstructedField>>,
}

/// Value represents a return result of a [igConstructedField's](igConstructedField) value. (and gets around rust multiple manual trait issues)
pub trait MetaFieldValue: Any + Display + Sync + Send {
    /// Allows you to downcast via `as_any().downcast_ref::<T>()`
    fn as_any(&self) -> &dyn Any;

    /// Allows you to downcast via `as_any().downcast_ref::<T>()`
    fn as_mut_any(&self) -> &mut dyn Any;
}

/// I don't have much to say on this one. Please check impl functions for usages you may like. Any igObject you want to use should probably not let you get this deep. This is getting deep into the metadata system. Please implement types you use!
pub struct igConstructedField {
    pub name: Arc<str>,
    /// Internal to ig-library. Stores the metafield's type as a string for serialization/deserialization later
    pub(crate) metafield: Box<dyn MetaFieldImpl>,
}

impl Display for igConstructedField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let current_value = self.meta_field_value();
        write!(
            f,
            "{} (type: {}, value: {})",
            self.name.as_ref(),
            type_name_of_val(&current_value),
            current_value
        )
    }
}

impl igConstructedField {
    /// Queries the metafield for its current value at the moment. Up to the user to cast this into the correct value
    fn meta_field_value(&self) -> &dyn MetaFieldValue {
        self.metafield.value()
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
    parent: Option<Arc<igMetaObject>>,
    field_storage: FieldStorage,
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

        let field_storage = self.get_current_fields(&self.platform, &parent_meta, object);

        igMetaObject {
            name: Arc::from(type_name),
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
        parent: &Option<Arc<igMetaObject>>,
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
