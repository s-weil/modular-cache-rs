use crate::{
    cache::{Cache, GetKey, GetKeyMut, KeyRegistry},
    key::KeyExtension,
};
use std::{
    hash::Hash,
    sync::{Arc, RwLock},
};

// TODO: use different concurrent primitives via features

// config(no(parking-lot), no(tokio))
pub struct ConcurrentCache<K, KeyReg, KeyExt, V>
where
    K: Eq + Hash,
    KeyReg: KeyRegistry<K, KeyExtension = KeyExt>,
{
    inner: RwLock<Cache<K, KeyReg, KeyExt, Arc<V>>>,
}

impl<K, KeyReg, KeyExt, V> ConcurrentCache<K, KeyReg, KeyExt, V>
where
    K: Eq + Hash + Clone,
    KeyReg: KeyRegistry<K, KeyExtension = KeyExt> + GetKey<K>,
{
    /// Get the key's value _without_ updating its statistics.
    /// Use `get_mut` in case the latter is essential.
    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        let guard = self.inner.read().unwrap();
        guard.get(key).cloned()
    }
}

impl<K, KeyReg, KeyExt, V> ConcurrentCache<K, KeyReg, KeyExt, V>
where
    K: Eq + Hash + Clone,
    KeyReg: KeyRegistry<K, KeyExtension = KeyExt> + GetKeyMut<K>,
{
    // TODO: rename: 'mut' is misleading
    /// Gets the key's value and updates its statistics. Locks this cache, blocking the current thread until it can be acquired.
    pub fn get_mut(&mut self, key: &K) -> Option<Arc<V>> {
        let mut guard = self.inner.write().unwrap();
        guard.get_mut(key).cloned()
    }
}

impl<K, KeyReg, KeyExt, V> ConcurrentCache<K, KeyReg, KeyExt, V>
where
    K: Eq + Hash + Clone,
    KeyReg: KeyRegistry<K, KeyExtension = KeyExt>,
    KeyExt: KeyExtension<K> + Clone,
{
    pub fn new(max_capacity: Option<usize>) -> Self {
        Self {
            inner: RwLock::new(Cache::new(max_capacity)),
        }
    }

    pub fn len(&self) -> usize {
        let guard = self.inner.read().unwrap();
        guard.len()
    }

    pub fn is_empty(&self) -> bool {
        let guard = self.inner.read().unwrap();
        guard.is_empty()
    }

    pub fn clear(&mut self) {
        let mut guard = self.inner.write().unwrap();
        guard.clear()
    }

    pub fn insert(&self, key: KeyExt, value: V) -> Option<Arc<V>> {
        let mut guard = self.inner.write().unwrap();
        guard.insert(key, Arc::new(value))
    }

    pub fn remove(&self, key: &K) -> Option<Arc<V>> {
        let mut guard = self.inner.write().unwrap();
        guard.remove(key)
    }
}
