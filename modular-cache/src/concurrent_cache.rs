use crate::cache::{Cache, KeyRegistry};
use std::{
    hash::Hash,
    sync::{Arc, RwLock},
};

// TODO: use different concurrent primitives via features

// config(no(parking-lot), no(tokio))
pub struct ConcurrentCache<K, R, V>
where
    K: Eq + Hash,
    R: KeyRegistry<K>,
{
    inner: RwLock<Cache<K, R, Arc<V>>>,
}

impl<K, S, V> ConcurrentCache<K, S, V>
where
    K: Eq + Hash + Clone,
    S: KeyRegistry<K>,
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

    pub fn clear(&mut self) {
        let mut guard = self.inner.write().unwrap();
        guard.clear()
    }

    /// Gets the key's value
    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        let guard = self.inner.read().unwrap();
        guard.get(key).cloned()
    }

    // TODO: rename: 'mut' is misleading
    /// Gets the key's value. Locks this cache, blocking the current thread until it can be acquired.
    pub fn get_mut(&mut self, key: &K) -> Option<Arc<V>> {
        let mut guard = self.inner.write().unwrap();
        guard.get_mut(key).cloned()
    }

    pub fn insert(&self, key: K, value: V) -> Option<Arc<V>> {
        let mut guard = self.inner.write().unwrap();
        guard.insert(key, Arc::new(value))
    }

    pub fn remove(&self, key: &K) -> Option<(K, Arc<V>)> {
        let mut guard = self.inner.write().unwrap();
        guard.remove(key)
    }
}