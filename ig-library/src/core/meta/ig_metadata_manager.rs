use crate::core::meta::ig_xml_metadata::{MetaEnum, MetaField, MetaObject};
use std::collections::HashMap;

pub struct igMetadataManager {
    pub meta_fields: HashMap<String, MetaField>,
    pub meta_enums: HashMap<String, MetaEnum>,
    pub meta_objects: HashMap<String, MetaObject>,
}

impl igMetadataManager {
    /// Initializes a new [igMetadataManager]. Types here are converted into maps early in order to save on lookup cost later
    pub fn new(
        field_list: Vec<MetaField>,
        enum_list: Vec<MetaEnum>,
        object_list: Vec<MetaObject>,
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
            meta_fields,
            meta_enums,
            meta_objects,
        }
    }
}
