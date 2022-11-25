use crate::cache::{Cache, GetKey, GetKeyMut, KeyRegistry};
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
    S: KeyRegistry<K> + GetKey<K>,
{
    /// Get the key's value _without_ updating its statistics.
    /// Use `get_mut` in case the latter is essential.
    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        let guard = self.inner.read().unwrap();
        guard.get(key).cloned()
    }
}

impl<K, S, V> ConcurrentCache<K, S, V>
where
    K: Eq + Hash + Clone,
    S: KeyRegistry<K> + GetKeyMut<K>,
{
    // TODO: rename: 'mut' is misleading
    /// Gets the key's value and updates its statistics. Locks this cache, blocking the current thread until it can be acquired.
    pub fn get_mut(&mut self, key: &K) -> Option<Arc<V>> {
        let mut guard = self.inner.write().unwrap();
        guard.get_mut(key).cloned()
    }
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

    pub fn is_empty(&self) -> bool {
        let guard = self.inner.read().unwrap();
        guard.is_empty()
    }

    pub fn clear(&mut self) {
        let mut guard = self.inner.write().unwrap();
        guard.clear()
    }

    pub fn insert(&self, key: K, value: V) -> Option<Arc<V>> {
        let mut guard = self.inner.write().unwrap();
        guard.insert(key, Arc::new(value))
    }

    pub fn remove(&self, key: &K) -> Option<Arc<V>> {
        let mut guard = self.inner.write().unwrap();
        guard.remove(key)
    }
}
