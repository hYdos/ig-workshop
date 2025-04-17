use crate::core::ig_archive::igArchive;

pub type igTObjectList<T> = Vec<T>; // We don't know types completely so this is kinda useless for us
pub type igArchiveList = igTObjectList<igArchive>;
