use std::time::Instant;

pub trait KeyExtension<K> {
    // type Key = K;
    fn key(&self) -> &K;
}

// TODO macro
// impl<K> KeyExtension<K> for K {
//     fn key(&self) -> &K {
//         &self
//     }
// }

impl KeyExtension<i32> for i32 {
    fn key(&self) -> &i32 {
        &self
    }
}

impl KeyExtension<usize> for usize {
    fn key(&self) -> &usize {
        &self
    }
}

#[derive(Clone, Debug)]
pub struct TimedKey<K> {
    key: K,
    instant: Instant,
}

impl<K> TimedKey<K> {
    pub fn create_now(key: K) -> Self {
        Self {
            key,
            instant: Instant::now(),
        }
    }
}

impl<K> KeyExtension<K> for TimedKey<K> {
    // type Key = K;
    fn key(&self) -> &K {
        &self.key
    }
}
