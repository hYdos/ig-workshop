use std::any::Any;
use crate::core::ig_archive::igArchive;
use crate::core::ig_objects::{igObject, igObjectDirectory};
use crate::util::ig_name::igName;
use log::error;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};
use crate::core::ig_memory::igMemoryPool;
use crate::core::meta::ig_metadata_manager::{__internalObjectBase, igMetaObject, FieldDoesntExist, SetObjectFieldError};

#[derive(Clone)]
pub struct igTObjectList<T>(pub Arc<RwLock<Vec<T>>>);

pub type igObjectList = igTObjectList<igObject>;
pub type igArchiveList = igTObjectList<Arc<igArchive>>;
pub type igObjectDirectoryList = igTObjectList<Arc<RwLock<igObjectDirectory>>>;
pub type igNameList = igTObjectList<Arc<igName>>;

pub struct QueryGuard<'a, T>(RwLockReadGuard<'a, Vec<T>>);
pub struct MutableQueryGuard<'a, T>(RwLockWriteGuard<'a, Vec<T>>);

impl<T: Send + Sync> __internalObjectBase for igTObjectList<T> {
    fn meta_type(&self) -> Arc<igMetaObject> {
        todo!()
    }

    fn internal_pool(&self) -> &igMemoryPool {
        todo!()
    }

    fn set_pool(&mut self, pool: igMemoryPool) {
        todo!()
    }

    fn set_field(&mut self, name: &str, value: Arc<RwLock<dyn Any + Send + Sync>>) -> Result<(), SetObjectFieldError> {
        todo!()
    }

    fn get_field(&self, name: &str) -> Result<Arc<RwLock<dyn Any + Send + Sync>>, FieldDoesntExist> {
        todo!()
    }

    fn as_any(&self) -> &(dyn Any + Send + Sync) {
        todo!()
    }

    fn as_mut_any(&mut self) -> &mut (dyn Any + Send + Sync) {
        todo!()
    }
}

impl<T: Clone> igTObjectList<T> {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self(Arc::new(RwLock::new(Vec::with_capacity(capacity))))
    }

    pub fn len(&self) -> usize {
        self.0.read().unwrap().len()
    }

    pub fn push(&self, value: T) {
        match self.0.try_write() {
            Ok(mut guard) => {
                guard.push(value);
            }
            Err(TryLockError::WouldBlock) => {
                error!("TryLockError. Pushing to igTObjectList would block the thread. We should always be thread safe anyway?");
                panic!("Alchemy Error! Broken state :( message hydos but this shouldn't ever happen anyway")
            }
            Err(TryLockError::Poisoned(_p)) => {
                error!("Error. igTObjectList was poisoned. The igTObjectList has a chance of being corrupt. We are in a broken state :(");
                panic!("Alchemy Error! Broken state :( message hydos but this shouldn't ever happen anyway")
            }
        }
    }

    pub fn push_blocking(&self, value: T) {
        match self.0.write() {
            Ok(mut guard) => {
                guard.push(value);
            }
            Err(_e) => {
                error!("Error. igTObjectList was poisoned. The igTObjectList has a chance of being corrupt. We are in a broken state :(");
                panic!("Alchemy Error! Broken state :( message hydos but this shouldn't ever happen anyway")
            }
        }
    }

    /// Used for iterating an indexing of a list
    pub fn query(&self) -> QueryGuard<T> {
        QueryGuard(self.0.read().unwrap())
    }

    /// Will force wait until a write lock can be taken. Drop this as soon as you can or use it sparingly please.
    pub fn query_mut(&self) -> MutableQueryGuard<T> {
        MutableQueryGuard(self.0.write().unwrap())
    }

    pub fn iter(&self) -> QueryGuardIter<T> {
        QueryGuardIter {
            query_guard: self.query(),
            idx: 0,
        }
    }

    pub fn get(&self, idx: usize) -> Option<T> {
        self.0.read().unwrap().get(idx).cloned()
    }
}

impl<'a, T: Clone> std::ops::Index<usize> for QueryGuard<'a, T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        &self.0[i]
    }
}

impl<'a, T: Clone> std::ops::Index<usize> for MutableQueryGuard<'a, T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        &self.0[i]
    }
}

impl<'a, T: Clone> std::ops::IndexMut<usize> for MutableQueryGuard<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub struct QueryGuardIter<'a, T> {
    query_guard: QueryGuard<'a, T>,
    idx: usize,
}

impl<'a, T: Clone> IntoIterator for &'a igTObjectList<T> {
    type Item = T;

    type IntoIter = QueryGuardIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: Clone> Iterator for QueryGuardIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.query_guard.0.len() {
            let item = &self.query_guard.0[self.idx];
            self.idx += 1;
            Some(item.clone())
        } else {
            None
        }
    }
}
