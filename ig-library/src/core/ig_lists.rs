use std::sync::{Arc, RwLock};
use crate::core::ig_archive::igArchive;
use crate::core::ig_objects::igObjectDirectory;
use crate::util::ig_name::igName;

pub type igTObjectList<T> = Vec<T>; // We don't know types completely so this is kinda useless for us
pub type igArchiveList = igTObjectList<Arc<igArchive>>;
pub type igObjectDirectoryList = igTObjectList<Arc<RwLock<igObjectDirectory>>>;
pub type igNameList = igTObjectList<Arc<igName>>;
