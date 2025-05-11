/// Custom implementations of objects to make usage more ergonomic. Currently just focused around objects using igDataList
use crate::core::ig_archive::igArchive;
use crate::core::ig_memory::igMemoryPool;
use crate::core::ig_objects::{igAny, igObject, igObjectDirectory, ObjectExt};
use crate::core::memory::igMemory;
use crate::core::meta::ig_metadata_manager::{
    __internalObjectBase, igMetaInstantiationError, igMetaObject, FieldDoesntExist,
    SetObjectFieldError,
};
use crate::util::ig_name::igName;
use log::{error, info, warn};
use std::any::Any;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};

#[derive(Clone)]
pub struct igDataList<T> {
    pub list: Arc<RwLock<Vec<T>>>,
    meta: Option<Arc<igMetaObject>>,
    pool: igMemoryPool,
}

pub type igObjectList = igDataList<igObject>;
pub type igArchiveList = igDataList<Arc<igArchive>>;
pub type igObjectDirectoryList = igDataList<Arc<RwLock<igObjectDirectory>>>;
pub type igNameList = igDataList<Arc<igName>>;

pub struct QueryGuard<'a, T>(RwLockReadGuard<'a, Vec<T>>);
pub struct MutableQueryGuard<'a, T>(RwLockWriteGuard<'a, Vec<T>>);

pub trait CastTo<T> {
    type Error;
    fn cast_to(self) -> Result<Arc<RwLock<T>>, Self::Error>;
}

impl<T> CastTo<T> for Arc<RwLock<dyn __internalObjectBase>>
where
    T: __internalObjectBase + 'static,
{
    type Error = igMetaInstantiationError;

    fn cast_to(self) -> Result<Arc<RwLock<T>>, Self::Error> {
        if let Some(t) = self.clone().downcast::<T>() {
            Ok(t)
        } else {
            // grab actual type name for error
            let actual = self.read().unwrap().meta_type().name.clone();
            Err(igMetaInstantiationError::TypeMismatchError(actual))
        }
    }
}

pub trait DataListExt<T> {
    /// Rebuild this `Arc<RwLock<igDataList<T>>>` into an
    /// `Arc<RwLock<igDataList<U>>>` by mapping each `&T` to a `U`.
    fn cast<U, F>(self, f: F) -> Arc<RwLock<igDataList<U>>>
    where
        U: Send + Sync + 'static,
        F: Fn(&T) -> U + Send + Sync + 'static;
}

impl<T> DataListExt<T> for Arc<RwLock<igDataList<T>>>
where
    T: Send + Sync + 'static,
{
    fn cast<U, F>(self, f: F) -> Arc<RwLock<igDataList<U>>>
    where
        U: Send + Sync + 'static,
        F: Fn(&T) -> U + Send + Sync + 'static,
    {
        // 1) grab the old list
        let old = self.read().unwrap();
        let items = old.list.read().unwrap();

        // 2) map each element
        let new_vec: Vec<U> = items.iter().map(|t| f(t)).collect();

        // 3) build a brand-new igDataList<U> with the same meta+pool
        let new_list = igDataList {
            list: Arc::new(RwLock::new(new_vec)),
            meta: old.meta.clone(),
            pool: old.pool,
        };

        // 4) wrap back in Arc<RwLock<…>>
        Arc::new(RwLock::new(new_list))
    }
}

impl<T: Send + Sync + 'static> __internalObjectBase for igDataList<T> {
    fn meta_type(&self) -> Arc<igMetaObject> {
        if self.meta.is_none() {
            panic!("Tried to get metadata of type constructed without one")
        }
        self.meta.clone().unwrap()
    }

    fn internal_pool(&self) -> &igMemoryPool {
        &self.pool
    }

    fn set_pool(&mut self, pool: igMemoryPool) {
        self.pool = pool;
    }

    fn set_field(&mut self, name: &str, value: Option<igAny>) -> Result<(), SetObjectFieldError> {
        if let Some(value) = value {
            match name {
                "_data" => {
                    let mut guard = value.write().unwrap();
                    let memory = guard.downcast_mut::<igMemory<_>>().unwrap();
                    info!("input list size: {}", memory.data.len());
                    info!("input list capacity: {}", memory.data.capacity());
                    self.list.write().unwrap().append(&mut memory.data);
                    return Ok(());
                }
                &_ => {
                    warn!(
                        "igDataList<T> attempted to set unknown field with name {} ",
                        name
                    );
                }
            }
        }

        Ok(())
    }

    fn get_non_null_field(&self, _name: &str) -> Result<igAny, FieldDoesntExist> {
        todo!()
    }

    fn get_field(
        &self,
        _name: &str,
    ) -> Result<Option<Arc<RwLock<(dyn Any + Send + Sync + 'static)>>>, FieldDoesntExist> {
        todo!()
    }

    fn as_any(&self) -> &(dyn Any + Send + Sync) {
        self
    }

    fn as_mut_any(&mut self) -> &mut (dyn Any + Send + Sync) {
        self
    }
}

impl<T: Send + Sync + 'static> igDataList<T> {
    pub fn construct(
        meta: Arc<igMetaObject>,
        pool: igMemoryPool,
    ) -> Result<Arc<RwLock<dyn __internalObjectBase>>, igMetaInstantiationError> {
        Ok(Arc::new(RwLock::new(igDataList {
            list: Arc::new(RwLock::new(Vec::<T>::new())),
            meta: Some(meta),
            pool,
        })))
    }
}

impl<T: Clone> igDataList<T> {
    pub fn new() -> Self {
        Self {
            list: Arc::new(RwLock::new(Vec::new())),
            meta: None,
            pool: Default::default(),
        }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            list: Arc::new(RwLock::new(Vec::with_capacity(capacity))),
            meta: None,
            pool: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.list.read().unwrap().len()
    }

    pub fn push(&self, value: T) {
        match self.list.try_write() {
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
        match self.list.write() {
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
        QueryGuard(self.list.read().unwrap())
    }

    /// Will force wait until a write lock can be taken. Drop this as soon as you can or use it sparingly please.
    pub fn query_mut(&self) -> MutableQueryGuard<T> {
        MutableQueryGuard(self.list.write().unwrap())
    }

    pub fn iter(&self) -> QueryGuardIter<T> {
        QueryGuardIter {
            query_guard: self.query(),
            idx: 0,
        }
    }

    pub fn get(&self, idx: usize) -> Option<T> {
        self.list.read().unwrap().get(idx).cloned()
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

impl<'a, T: Clone> IntoIterator for &'a igDataList<T> {
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
