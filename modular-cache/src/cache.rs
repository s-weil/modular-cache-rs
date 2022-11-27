use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::key::KeyExtension;

pub trait GetKey<K>: Sized {
    /// Gets the key's value _without_ updating its statistics.
    /// This is crucial for instance for a
    /// [`LRU cache`](https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU)
    /// and should be considered to not be implemented in this case.
    fn get(&self, key: &K) -> Option<&K>;
}

pub trait GetKeyMut<K>: Sized {
    // Get the key's value and updates its statistics.
    fn get(&mut self, key: &K) -> Option<&K>;
}

pub trait HouseKeeper<K> {
    /// Make sure keys are still valid; return invalidated ones.
    fn house_keeping(&mut self) -> Option<HashSet<K>>;
}

// TODO: split into smaller traits
pub trait KeyRegistry<K>: Sized {
    // type Key;
    type KeyExtension: KeyExtension<K>; //Self::Key>;

    fn with_capacity(max_capacity: usize) -> Self;

    fn init(max_capacity: Option<usize>) -> Self {
        let mc = max_capacity.unwrap_or(usize::MAX);
        Self::with_capacity(mc)
    }

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn clear(&mut self);

    // return deleted key (if some)
    fn add_or_update(&mut self, key: Self::KeyExtension) -> Option<K>;

    fn try_remove(&mut self, key: &K) -> Option<K>;
}

// TODO: could also have a trait for cache and then inject LRU, etc
// TODO: generalize with randomstate and buildhasher
// TODO: stats (instant) within `orderd_keys`?
pub struct Cache<K, KeyReg, KeyExt, V>
where
    K: Eq + Hash,
    KeyReg: KeyRegistry<K, KeyExtension = KeyExt>,
{
    store: HashMap<K, V>,
    key_registry: KeyReg,
}

impl<K, KeyReg, KeyExt, V> Cache<K, KeyReg, KeyExt, V>
where
    K: Eq + Hash + Clone,
    KeyReg: KeyRegistry<K, KeyExtension = KeyExt> + GetKey<K>,
{
    /// Get the key's value _without_ updating its statistics.
    /// Use `get_mut` in case the latter is of the essence.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.key_registry.get(key).and_then(|k| self.store.get(k))
    }
}

impl<K, KeyReg, KeyExt, V> Cache<K, KeyReg, KeyExt, V>
where
    K: Eq + Hash + Clone,
    KeyReg: KeyRegistry<K, KeyExtension = KeyExt> + GetKeyMut<K>,
{
    /// Get the key's value and updates its statistics
    pub fn get_mut(&mut self, key: &K) -> Option<&V> {
        self.key_registry.get(key).and_then(|k| self.store.get(k))
    }
}

impl<K, KeyReg, KeyExt, V> Cache<K, KeyReg, KeyExt, V>
where
    K: Eq + Hash + Clone,
    KeyReg: KeyRegistry<K, KeyExtension = KeyExt>,
    KeyExt: KeyExtension<K>,
{
    pub fn new(max_capacity: Option<usize>) -> Self {
        Self {
            store: HashMap::new(),
            key_registry: KeyReg::init(max_capacity),
        }
    }

    /// Clears the registry and store, removing all key-value pairs.
    /// Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.key_registry.clear();
        self.store.clear();
    }

    pub fn len(&self) -> usize {
        // TODO: check also key_registry to be synced
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        // TODO: check also key_registry to be synced
        self.store.is_empty()
    }

    /// Inserts a key-value pair into the cache.
    /// If the cache did not have this key present, None is returned.
    /// If the cache did have this key present, the value is updated, and the old value is returned.
    /// TODO: remove or keep? The key is not updated, though; this matters for types that can be == without being identical.
    /// See the module-level documentation for more.
    pub fn insert(&mut self, key: KeyExt, value: V) -> Option<V>
    where
        KeyExt: Clone,
    {
        if let Some(deleted_key) = self.key_registry.add_or_update(key.clone()) {
            self.store.remove(&deleted_key);
        }
        self.store.insert(key.key().clone(), value)
    }

    /// TODO: make sure keys are aligne within registry and store
    /// Removes a key from the cache, returning the value at the key if the key was previously in the cache.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.key_registry
            .try_remove(key)
            .and_then(|k| self.store.remove(&k))
    }
}
