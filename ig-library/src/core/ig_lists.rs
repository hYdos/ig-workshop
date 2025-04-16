use crate::core::ig_archive::igArchive;
use std::collections::VecDeque;

pub type igTObjectList<T> = igTDataList<T>; // We don't know types completely so this is kinda useless for us
pub type igArchiveList = igTObjectList<igArchive>;

pub struct igTDataList<T> {
    pub items: VecDeque<T>,
}

impl<T> igTDataList<T> {

    pub fn new() -> igTDataList<T> {
        igTDataList {
            items: VecDeque::new(),
        }
    }
}
