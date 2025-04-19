use std::cell::RefCell;
use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use log::info;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

type igMetaFieldXml = Vec<MetaField>;

#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub platform: IG_CORE_PLATFORM,
    pub align: u16,
    pub size: u16,
}

#[derive(Debug, Clone)]
pub struct MetaField {
    pub name: String,
    pub platform_info: Vec<PlatformInfo>,
}

type igMetaEnumXml = Vec<MetaEnum>;

#[derive(Debug, Clone)]
pub struct MetaEnumValue {
    pub name: String,
    pub value: i32,
}

#[derive(Debug, Clone)]
pub struct MetaEnum {
    pub ref_name: String,
    pub values: Vec<MetaEnumValue>,
}

type igMetaObjectXml = Vec<MetaObject>;

/// Stores extra information about what the hash table expects
#[derive(Debug, Clone)]
pub struct HashTableInfo {
    pub invalid_value: String,
    pub invalid_key: String,
}

#[derive(Debug, Clone)]
pub struct BitShiftInfo {
    pub shift: u8,
    pub bits: u8,
    /// The name of the field to store the result at
    pub storage_field: String,
    /// The type of the new data created with the bits. Once data is finalized this will always be present
    pub _type: Option<MetaObjectField>,
}

/// Stores the meta object inside a igPropertyFieldMetaField
type PropertyInfo = MetaObjectField;

type MetaObjectField = Arc<RwLock<RawMetaObjectField>>;

#[derive(Debug, Clone)]
pub struct VectorInfo {
    /// Technically possible to store a very complex MetaObjectField. I personally recommend staying away from these areas for your own sake. Will always be Some after reading is finalized
    pub field: Option<MetaObjectField>,
    /// Stores the multiple that all alignment should follow for the members of this type?
    pub mem_type_alignment_multiple: u8,
}

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
    /// Some fields will require a specific alignment otherwise they won't work. These types will specify it. I am unsure specifically which ones do this.
    pub required_alignment: Option<u8>,
    /// Present when _type is equal to "igVectorMetaField"
    pub ig_vector_info: Option<VectorInfo>,
    /// Present when _type is equal to "igMemoryRefMetaField"
    pub ig_memory_ref_info: Option<MetaObjectField>,
    /// Present when _type is equal to "igBitFieldMetaField"
    pub ig_bit_shift_info: Option<Arc<RwLock<BitShiftInfo>>>,
    /// Present when _type is equal to "igPropertyFieldMetaField"
    pub ig_property_info: Option<PropertyInfo>,
    /// Present when _type is equal to "igEnumMetaField". Stores the meta enum to use with the field
    pub ig_meta_enum: Option<String>,
    /// Present when _type is equal to "igStaticMetaField"
    pub ig_static_info: Option<MetaObjectField>,
}

#[derive(Debug, Clone)]
pub struct MetaObject {
    /// The type of meta object. for the most part, this will always be "igMetaObject" and I don't believe it has a real use.
    pub _type: String,
    /// Name associated
    pub ref_name: String,
    /// The parent of the current object's _type. As far as I know this is present on every object apart from __internalObjectBase
    pub base_type: Option<String>,
    /// Present when base_type is present and extends an object extending "igObjectList" (seen in tfb script) or "igObjectList" itself
    pub object_list_type: Option<String>,
    /// Present when base_type is present and extends an object extending "igHashTable" or "igHashTable" itself
    pub hash_table_info: Option<HashTableInfo>,
    /// New fields added by the current meta object
    pub new_fields: Vec<MetaObjectField>,
    /// Fields from the parent that are replaced by new ones.
    pub overriden_fields: Vec<MetaObjectField>,
    /// Present when base_type is present and extends an object extending "igCompoundMetaField" or "igCompoundMetaField" itself
    pub compound_fields: Vec<MetaObjectField>,
}

pub fn load_xml_metadata(
    meta_directory: PathBuf,
) -> Result<(igMetaFieldXml, igMetaEnumXml, igMetaObjectXml), String> {
    let _meta_enum_path = meta_directory.join("metaenums.xml");
    let _meta_field_path = meta_directory.join("metafields.xml");
    let _meta_object_path = meta_directory.join("metaobjects.xml");

    Ok((
        load_meta_fields(&_meta_field_path)?,
        load_meta_enums(&_meta_enum_path)?,
        load_meta_objects(&_meta_object_path)?,
    ))
}

pub enum FieldType {
    NewField,
    OverridenField,
    CompoundField,
}

fn load_meta_fields(path: &PathBuf) -> Result<Vec<MetaField>, String> {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut buf = Vec::new();
    let mut reader = Reader::from_file(path).map_err(|e| e.to_string())?;
    reader.config_mut().trim_text(true);

    let mut meta_fields: Vec<MetaField> = Vec::new();
    let mut current_meta_field_name: Option<String> = None;
    let mut platform_info_buffer: Vec<PlatformInfo> = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => {
                panic!("at position {}: {:?}", reader.error_position(), e)
            }
            Ok(Event::Eof) => break,
            Ok(Event::Empty(e)) => {
                let mut platform: Option<IG_CORE_PLATFORM> = None;
                let mut size: Option<u16> = None;
                let mut align: Option<u16> = None;

                for result in e.attributes() {
                    let attrib = result.unwrap();
                    match attrib.key.local_name().as_ref() {
                        b"platform" => {
                            let raw = String::from(attrib.unescape_value().unwrap());
                            let err_msg = format!("Alchemy Error: Parsing metafields.xml failed. Unknown platform {}", raw);
                            platform = Some(IG_CORE_PLATFORM::from_str(raw.as_str()).expect(&err_msg));
                        }
                        b"size" => {
                            let raw = String::from(attrib.unescape_value().unwrap());
                            let without_prefix = raw.trim_start_matches("0x");
                            size = Some(u16::from_str_radix(without_prefix, 16).unwrap());
                        }
                        b"align" => {
                            let raw = String::from(attrib.unescape_value().unwrap());
                            let without_prefix = raw.trim_start_matches("0x");
                            align = Some(u16::from_str_radix(without_prefix, 16).unwrap());
                        }
                        _ => panic!("Alchemy Error: Parsing metafields.xml failed. Unknown attribute present. Are we out of date?")
                    }
                }

                platform_info_buffer.push(PlatformInfo {
                    platform: platform.unwrap(),
                    align: align.unwrap(),
                    size: size.unwrap(),
                })
            }
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"metafield" => {
                    for result in e.attributes() {
                        let attrib = result.unwrap();
                        current_meta_field_name =
                            Some(String::from(attrib.unescape_value().unwrap()));
                    }
                }
                b"platforminfo" => {}
                _ => (),
            },
            Ok(Event::End(e)) => match e.local_name().as_ref() {
                b"metafield" => {
                    let metafield_name = current_meta_field_name.clone().expect(
                        "Alchemy Error: Parsing metafields.xml failed. Is metafields.xml valid xml?",
                    );

                    meta_fields.push(MetaField {
                        name: metafield_name,
                        platform_info: platform_info_buffer.to_vec(),
                    });

                    current_meta_field_name = None;
                    platform_info_buffer.clear();
                }
                &_ => {}
            },
            _ => {}
        }
    }

    buf.clear();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    info!("metafields.xml loaded in {}ms", (end - start).as_millis());
    Ok(meta_fields)
}

fn load_meta_enums(path: &PathBuf) -> Result<Vec<MetaEnum>, String> {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut buf = Vec::new();
    let mut reader = Reader::from_file(path).map_err(|e| e.to_string())?;
    reader.config_mut().trim_text(true);

    let mut meta_enums: Vec<MetaEnum> = Vec::new();
    let mut current_meta_enum_name: Option<String> = None;
    let mut value_buffer: Vec<MetaEnumValue> = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => {
                panic!("at position {}: {:?}", reader.error_position(), e)
            }
            Ok(Event::Eof) => break,
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"value" => {
                    let mut name: Option<String> = None;
                    let mut value: Option<i32> = None;

                    for result in e.attributes() {
                        let attrib = result.unwrap();
                        match attrib.key.local_name().as_ref() {
                            b"name" => {
                                name = Some(String::from(attrib.unescape_value().unwrap()));
                            }
                            b"value" => {
                                let raw = String::from(attrib.unescape_value().unwrap());
                                value = Some(raw.parse::<i32>().unwrap());
                            }
                            _ => return Err(format!("Attribute {} was present, but it wasn't expected. are we out of date?", String::from_utf8_lossy(attrib.key.local_name().as_ref()))),
                        }
                    }

                    value_buffer.push(MetaEnumValue {
                        name: name.unwrap(),
                        value: value.unwrap(),
                    })
                }
                _ => (),
            },
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"metaenum" => {
                    for result in e.attributes() {
                        let attrib = result.unwrap();
                        current_meta_enum_name =
                            Some(String::from(attrib.unescape_value().unwrap()));
                    }
                }
                _ => (),
            },
            Ok(Event::End(e)) => match e.local_name().as_ref() {
                b"metaenum" => {
                    let meta_enum_name = current_meta_enum_name.clone().expect(
                        "Alchemy Error: Parsing metaenums.xml failed. Is metaenums.xml valid xml?",
                    );

                    meta_enums.push(MetaEnum {
                        ref_name: meta_enum_name,
                        values: value_buffer.to_vec(),
                    });

                    current_meta_enum_name = None;
                    value_buffer.clear();
                }
                &_ => {}
            },
            _ => {}
        }
    }

    buf.clear();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    info!("metaenums.xml loaded in {}ms", (end - start).as_millis());
    Ok(meta_enums)
}

// I got to talk to jasleen about simplifying this format because damn this is hard
fn load_meta_objects(path: &PathBuf) -> Result<Vec<MetaObject>, String> {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut buf = Vec::new();
    let mut reader = Reader::from_file(path).map_err(|e| e.to_string())?;
    reader.config_mut().trim_text(true);

    let mut meta_objects = Vec::new();
    let mut current_meta_object: Option<Arc<RefCell<MetaObject>>> = None;
    let mut current_meta_field: Option<MetaObjectField> = None;
    // when reading "overriddenmetafields" this should be false but when reading "metafields" it should be true
    let mut field_type = FieldType::NewField;
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => {
                panic!("at position {}: {:?}", reader.error_position(), e)
            }
            Ok(Event::Eof) => break,
            Ok(Event::Empty(e)) => match e.local_name().as_ref() {
                b"overriddenmetafields" => field_type = FieldType::OverridenField,
                b"metafields" => field_type = FieldType::NewField,
                b"compoundfields" => field_type = FieldType::CompoundField,
                b"metafield" => on_metafield_tag(
                    &mut meta_objects,
                    &mut current_meta_object,
                    &mut current_meta_field,
                    &mut field_type,
                    &e,
                )?,
                _ => {}
            },
            Ok(Event::Start(e)) => on_metafield_tag(
                &mut meta_objects,
                &mut current_meta_object,
                &mut current_meta_field,
                &mut field_type,
                &e,
            )?,

            _ => {}
        }
    }

    buf.clear();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    info!("metaobjects.xml loaded in {}ms", (end - start).as_millis());
    Ok(meta_objects)
}

fn on_metafield_tag(
    meta_objects: &mut Vec<MetaObject>,
    current_meta_object: &mut Option<Arc<RefCell<MetaObject>>>,
    current_meta_field: &mut Option<MetaObjectField>,
    field_type: &mut FieldType,
    e: &BytesStart,
) -> Result<(), String> {
    match e.local_name().as_ref() {
        b"metaobject" => {
            if let Some(old_meta_obj) = current_meta_object.clone() {
                meta_objects.push(old_meta_obj.clone().borrow().to_owned());
            }

            let mut _type: Option<String> = None;
            let mut ref_name: Option<String> = None;
            let mut base_type: Option<String> = None;

            for result in e.attributes() {
                let attrib = result.unwrap();
                match attrib.key.local_name().as_ref() {
                    b"type" => {
                        _type = Some(String::from(attrib.unescape_value().unwrap()));
                    }

                    b"refname" => {
                        ref_name = Some(String::from(attrib.unescape_value().unwrap()));
                    }

                    b"basetype" => {
                        base_type = Some(String::from(attrib.unescape_value().unwrap()));
                    }
                    _ => {
                        // Ignore all the extra useless info
                    }
                }
            }

            *current_meta_object = Some(Arc::new(RefCell::new(MetaObject {
                _type: _type.unwrap(),
                ref_name: ref_name.unwrap(),
                base_type,
                new_fields: Vec::new(),
                overriden_fields: Vec::new(),
                compound_fields: Vec::new(),
                object_list_type: None,
                hash_table_info: None,
            })))
        }
        b"objectlist" => {
            for result in e.attributes() {
                let attrib = result.unwrap();
                match attrib.key.local_name().as_ref() {
                    b"elementtype" => {
                        current_meta_object
                            .clone()
                            .unwrap()
                            .borrow_mut()
                            .object_list_type =
                            Some(String::from(attrib.unescape_value().unwrap()));
                    }
                    _ => {
                        return Err(format!(
                            "objectlist: unknown attribute \"{}\" present. Are we out of date?",
                            String::from_utf8_lossy(attrib.key.local_name().as_ref())
                        ))
                    }
                }
            }
        }
        b"hashtable" => {
            let mut invalid_value: Option<String> = None;
            let mut invalid_key: Option<String> = None;

            for result in e.attributes() {
                let attrib = result.unwrap();
                match attrib.key.local_name().as_ref() {
                    b"invalid_value" => {
                        invalid_value = Some(String::from(attrib.unescape_value().unwrap()));
                    }
                    b"invalid_key" => {
                        invalid_key = Some(String::from(attrib.unescape_value().unwrap()));
                    }
                    _ => {
                        return Err(format!(
                            "hashtable: unknown attribute \"{}\" present. Are we out of date?",
                            String::from_utf8_lossy(attrib.key.local_name().as_ref())
                        ))
                    }
                }
            }

            current_meta_object
                .clone()
                .unwrap()
                .borrow_mut()
                .hash_table_info = Some(HashTableInfo {
                invalid_value: invalid_value.unwrap(),
                invalid_key: invalid_key.unwrap(),
            })
        }

        b"metafield" => {
            // Let's find out if we are a child first before we do anything.
            if current_meta_field.is_some() {
                let current_meta_field_ref = current_meta_field.clone().unwrap();
                let current_type = current_meta_field_ref.read().unwrap().clone()._type;

                match current_type.as_str() {
                    "igPropertyFieldMetaField" => {
                        let child_metafield = process_new_metafield(&e);
                        current_meta_field_ref.write().unwrap().ig_property_info = Some(
                            child_metafield
                                .clone()
                                .expect("Failed to process child field of igPropertyFieldMetaField"),
                        );

                        // TODO: de-duplicate this
                        if current_meta_field.is_some() {
                            let field = current_meta_field.clone().unwrap();
                            let raw_meta_object = current_meta_object.clone().unwrap();
                            let mut meta_object_borrow = raw_meta_object.borrow_mut();
                            let field_vector = match field_type {
                                FieldType::NewField => &mut meta_object_borrow.new_fields,
                                FieldType::OverridenField => {
                                    &mut meta_object_borrow.overriden_fields
                                }
                                FieldType::CompoundField => &mut meta_object_borrow.compound_fields,
                            };
                            field_vector.push(field);
                        }

                        *current_meta_field = child_metafield;
                    }

                    "igStaticMetaField" => {
                        let child_metafield = process_new_metafield(&e);
                        current_meta_field_ref.write().unwrap().ig_static_info = Some(
                            child_metafield
                                .clone()
                                .expect("Failed to process child field of igStaticMetaField"),
                        );

                        // TODO: de-duplicate this
                        if current_meta_field.is_some() {
                            let field = current_meta_field.clone().unwrap();
                            let raw_meta_object = current_meta_object.clone().unwrap();
                            let mut meta_object_borrow = raw_meta_object.borrow_mut();
                            let field_vector = match field_type {
                                FieldType::NewField => &mut meta_object_borrow.new_fields,
                                FieldType::OverridenField => {
                                    &mut meta_object_borrow.overriden_fields
                                }
                                FieldType::CompoundField => &mut meta_object_borrow.compound_fields,
                            };
                            field_vector.push(field);
                        }

                        *current_meta_field = child_metafield;
                    }

                    "igVectorMetaField" => {
                        let child_metafield = process_new_metafield(&e);
                        current_meta_field_ref
                            .write()
                            .unwrap()
                            .ig_vector_info
                            .as_mut()
                            .unwrap()
                            .field = Some(
                            child_metafield
                                .clone()
                                .expect("Failed to process child field of igVectorMetaField"),
                        );

                        // TODO: de-duplicate this
                        if current_meta_field.is_some() {
                            let field = current_meta_field.clone().unwrap();
                            let raw_meta_object = current_meta_object.clone().unwrap();
                            let mut meta_object_borrow = raw_meta_object.borrow_mut();
                            let field_vector = match field_type {
                                FieldType::NewField => &mut meta_object_borrow.new_fields,
                                FieldType::OverridenField => {
                                    &mut meta_object_borrow.overriden_fields
                                }
                                FieldType::CompoundField => &mut meta_object_borrow.compound_fields,
                            };
                            field_vector.push(field);
                        }

                        *current_meta_field = child_metafield;
                    }
                    "igBitFieldMetaField" => {
                        let child_metafield = process_new_metafield(&e);
                        current_meta_field_ref
                            .read()
                            .unwrap()
                            .ig_bit_shift_info
                            .as_ref()
                            .unwrap()
                            .write()
                            .unwrap()
                            ._type = Some(
                            child_metafield
                                .clone()
                                .expect("Failed to process child field of igVectorMetaField"),
                        );

                        // TODO: de-duplicate this
                        if current_meta_field.is_some() {
                            let field = current_meta_field.clone().unwrap();
                            let raw_meta_object = current_meta_object.clone().unwrap();
                            let mut meta_object_borrow = raw_meta_object.borrow_mut();
                            let field_vector = match field_type {
                                FieldType::NewField => &mut meta_object_borrow.new_fields,
                                FieldType::OverridenField => {
                                    &mut meta_object_borrow.overriden_fields
                                }
                                FieldType::CompoundField => &mut meta_object_borrow.compound_fields,
                            };
                            field_vector.push(field);
                        }

                        *current_meta_field = child_metafield;
                    }
                    "igMemoryRefMetaField" => {
                        // This ONE type is driving me insane.
                        // We ARE a child, but we are still burdened with the responsibility
                        // to process an entire new metafield which itself has a chance of having its own metafields...

                        let child_metafield = process_new_metafield(&e);
                        current_meta_field_ref.write().unwrap().ig_memory_ref_info = Some(
                            child_metafield
                                .clone()
                                .expect("Failed to process child field of igMemoryRefMetaField"),
                        );

                        if current_meta_field.is_some() {
                            let field = current_meta_field.clone().unwrap();
                            let raw_meta_object = current_meta_object.clone().unwrap();
                            let mut meta_object_borrow = raw_meta_object.borrow_mut();
                            let field_vector = match field_type {
                                FieldType::NewField => &mut meta_object_borrow.new_fields,
                                FieldType::OverridenField => {
                                    &mut meta_object_borrow.overriden_fields
                                }
                                FieldType::CompoundField => &mut meta_object_borrow.compound_fields,
                            };
                            field_vector.push(field);
                        }

                        *current_meta_field = child_metafield;
                    }
                    _ => {
                        // Not a child. Let's push it into the list
                        if current_meta_field.is_some() {
                            let field = current_meta_field.clone().unwrap();
                            let raw_meta_object = current_meta_object.clone().unwrap();
                            let mut meta_object_borrow = raw_meta_object.borrow_mut();
                            let field_vector = match field_type {
                                FieldType::NewField => &mut meta_object_borrow.new_fields,
                                FieldType::OverridenField => {
                                    &mut meta_object_borrow.overriden_fields
                                }
                                FieldType::CompoundField => &mut meta_object_borrow.compound_fields,
                            };
                            field_vector.push(field);
                        }

                        *current_meta_field = process_new_metafield(&e);
                    }
                }
            }

            // We aren't a child. we must be new. Save and get rid of the old type
            if current_meta_field.is_some() {
                let field = current_meta_field.clone().unwrap();
                let raw_meta_object = current_meta_object.clone().unwrap();
                let mut meta_object_borrow = raw_meta_object.borrow_mut();
                let field_vector = match field_type {
                    FieldType::NewField => &mut meta_object_borrow.new_fields,
                    FieldType::OverridenField => &mut meta_object_borrow.overriden_fields,
                    FieldType::CompoundField => &mut meta_object_borrow.compound_fields,
                };
                field_vector.push(field);
            }

            *current_meta_field = process_new_metafield(&e);
        }
        _ => {}
    }

    Ok(())
}

fn process_new_metafield(e: &BytesStart) -> Option<MetaObjectField> {
    let mut _type: Option<String> = None;
    let mut offset: Option<u16> = None;
    let mut name: Option<String> = None;
    let mut meta_object: Option<String> = None;
    let mut required_alignment: Option<u8> = None;
    let mut ig_bit_shift_info = BitShiftInfo {
        shift: 0,
        bits: 0,
        storage_field: "".to_string(),
        _type: None,
    };
    let mut ig_meta_enum: Option<String> = None;
    let mut ig_vector_info = VectorInfo {
        field: None,
        mem_type_alignment_multiple: u8::MAX,
    };

    for result in e.attributes() {
        let attrib = result.unwrap();
        match attrib.key.local_name().as_ref() {
            b"type" => _type = Some(String::from(attrib.unescape_value().unwrap())),
            b"offset" => {
                let raw = String::from(attrib.unescape_value().unwrap());
                let without_prefix = raw.trim_start_matches("0x");
                offset = Some(u16::from_str_radix(&without_prefix, 16).unwrap());
            }
            b"name" => name = Some(String::from(attrib.unescape_value().unwrap())),
            b"requiredAlignment" => {
                // Jasleen made it write the value in decimal not hex here?
                let raw = String::from(attrib.unescape_value().unwrap());
                required_alignment = Some(u8::from_str(&raw).unwrap());
            }
            // igBitFieldMetaField
            b"shift" => {
                let raw = String::from(attrib.unescape_value().unwrap());
                let without_prefix = raw.trim_start_matches("0x");
                ig_bit_shift_info.shift = u8::from_str_radix(&without_prefix, 16).unwrap();
            }
            b"bits" => {
                let raw = String::from(attrib.unescape_value().unwrap());
                let without_prefix = raw.trim_start_matches("0x");
                ig_bit_shift_info.bits = u8::from_str_radix(&without_prefix, 16).unwrap();
            }
            b"storageField" => {
                ig_bit_shift_info.storage_field = String::from(attrib.unescape_value().unwrap())
            }
            // igEnumMetaField
            b"metaenum" => ig_meta_enum = Some(String::from(attrib.unescape_value().unwrap())),
            // igObjectRefMetaField
            b"metaobject" => meta_object = Some(String::from(attrib.unescape_value().unwrap())),
            // igVectorMetaField
            b"memTypeAlignmentMultiple" => {
                let raw = String::from(attrib.unescape_value().unwrap());
                let without_prefix = raw.trim_start_matches("0x");
                ig_vector_info.mem_type_alignment_multiple =
                    u8::from_str_radix(&without_prefix, 16).unwrap();
            }
            // igMemoryRefMetaField
            // ...
            // igPropertyFieldMetaField
            // ...
            _ => {
                // Fields may be missed
            }
        }
    }

    // Don't store bit shift info when it's not a bit metafield to not confuse users of metadata
    let mut optional_ig_bit_shift = None;
    if ig_bit_shift_info.storage_field != "" {
        optional_ig_bit_shift = Some(Arc::new(RwLock::new(ig_bit_shift_info)))
    }

    // Don't store vector info when it's not a igVectorMetaField to not confuse users of metadata
    let mut optional_ig_vector = None;
    if _type.clone().unwrap().clone() == "igVectorMetaField" {
        optional_ig_vector = Some(ig_vector_info)
    }

    Some(Arc::new(RwLock::new(RawMetaObjectField {
        _type: _type.unwrap(),
        offset: offset.unwrap(),
        name,
        meta_object,
        required_alignment,
        ig_vector_info: optional_ig_vector, // Requires child metafield to get more information
        ig_memory_ref_info: None,           // Requires child metafield to get more information
        ig_bit_shift_info: optional_ig_bit_shift,
        ig_property_info: None, // Requires child metafield to get more information
        ig_meta_enum,
        ig_static_info: None, // Requires child metafield to get more information
    })))
}
