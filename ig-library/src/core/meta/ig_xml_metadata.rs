use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::Arc;

type igMetaFieldXml = Vec<MetaField>;

#[derive(Debug, Clone)]
pub struct PlatformInfo {
    platform: IG_CORE_PLATFORM,
    align: u16,
    size: u16,
}

#[derive(Debug, Clone)]
pub struct MetaField {
    name: String,
    platform_info: Vec<PlatformInfo>,
}

type igMetaEnumXml = Vec<MetaEnum>;

#[derive(Debug, Clone)]
pub struct MetaEnumValue {
    name: String,
    value: i32,
}

#[derive(Debug, Clone)]
pub struct MetaEnum {
    ref_name: String,
    values: Vec<MetaEnumValue>,
}

type igMetaObjectXml = Vec<MetaObject>;

/// Stores extra information about what the hash table expects
#[derive(Debug, Clone)]
pub struct HashTableInfo {
    invalid_value: String,
    invalid_key: String
}

type MetaObjectField = Arc<RefCell<RawMetaObjectField>>;
type TemplateArgs = MetaObjectField;

#[derive(Debug, Clone)]
pub struct RawMetaObjectField {
    /// meta field type to use when serializing, deserializing, and constructing new instances
    pub _type: String,
    /// offset in the object where the field resides
    pub offset: u16,
    /// Will most times be present unless it is not the parent field
    pub name: Option<String>,
    /// Present when _type is equal to "igObjectRefMetaField"
    pub meta_object: Option<String>,
    /// Present when _type is equal to "igVectorMetaField"
    pub ig_vector_info: Option<TemplateArgs>,
    /// Present when _type is equal to "igMemoryRefMetaField"
    pub ig_memory_ref_info: Option<MetaObjectField>
}

#[derive(Debug, Clone)]
pub struct MetaObject {
    pub _type: String,
    /// Name associated
    pub ref_name: String,
    /// The parent of the current object's _type
    pub base_type: Option<String>,
    /// Present when base_type is present and extends an object extending "igObjectList" (seen in tfb script) or "igObjectList" itself
    pub object_list_type: Option<String>,
    /// Present when base_type is present and extends an object extending "igHashTable" or "igHashTable" itself
    pub hash_table_info: Option<HashTableInfo>,
    /// New fields added by the current meta object
    pub new_fields: Vec<MetaObjectField>,
    /// Fields from the parent that are replaced by new ones.
    pub overriden_fields: Vec<MetaObjectField>,
}

pub fn load_xml_metadata(meta_directory: PathBuf) -> Result<(igMetaFieldXml, igMetaEnumXml, igMetaObjectXml), String> {
    let _meta_enum_path = meta_directory.join("metaenums.xml");
    let _meta_field_path = meta_directory.join("metafields.xml");
    let _meta_object_path = meta_directory.join("metaobjects.xml");

    todo!()
}