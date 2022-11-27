// generate data
extern crate modular_cache;

use std::sync::Arc;

use modular_cache::{
    cache::{GetKey, KeyRegistry},
    concurrent_cache::ConcurrentCache,
    timed_cache::{ConcurrentTimedCache, ConcurrentTimedCacheV2, TimedCache, TimedCacheV2},
};
use rand::Rng;

// Poem by Friedrich Schiller. The corresponding music is the European Anthem.
const ODE_AN_DIE_FREUDE: [&str; 32] = [
    "Freude schöner Götterfunken",
    "Tochter aus Elysium,",
    "Wir betreten feuertrunken,",
    "Himmlische, dein Heiligtum!",
    "Deine Zauber binden wieder",
    "Was die Mode streng geteilt;",
    "Alle Menschen werden Brüder,",
    "Wo dein sanfter Flügel weilt.",
    "Freude schöner Götterfunken",
    "Tochter aus Elysium,",
    "Wir betreten feuertrunken,",
    "Himmlische, dein Heiligtum!",
    "Deine Zauber binden wieder",
    "Was die Mode streng geteilt;",
    "Alle Menschen werden Brüder,",
    "Wo dein sanfter Flügel weilt.",
    "Freude schöner Götterfunken",
    "Tochter aus Elysium,",
    "Wir betreten feuertrunken,",
    "Himmlische, dein Heiligtum!",
    "Deine Zauber binden wieder",
    "Was die Mode streng geteilt;",
    "Alle Menschen werden Brüder,",
    "Wo dein sanfter Flügel weilt.",
    "Freude schöner Götterfunken",
    "Tochter aus Elysium,",
    "Wir betreten feuertrunken,",
    "Himmlische, dein Heiligtum!",
    "Deine Zauber binden wieder",
    "Was die Mode streng geteilt;",
    "Alle Menschen werden Brüder,",
    "Wo dein sanfter Flügel weilt.",
];

fn create_value(value_len: usize) -> String {
    let v = vec!["a"; value_len];
    v.join("")
}

/// Creates a collection of at most 32 keys
// pub(crate) fn gernerate_key_values(n_keys: usize, value_len: usize) -> Vec<(String, String)> {
//     let mut key_values = Vec::with_capacity(n_keys);
//     let mut rng = rand::thread_rng();

//     let v = create_value(value_len);

//     for _ in 0..n_keys {
//         let key = ODE_AN_DIE_FREUDE[rng.gen_range(0..32)].to_string();
//         key_values.push((key, v.clone()));
//     }

//     key_values
// }

// TODO: randomize access / shuffle key_values after inserted

pub(crate) fn gernerate_key_values(n_keys: usize, value_len: usize) -> Vec<(usize, String)> {
    let mut key_values = Vec::with_capacity(n_keys);

    let v = create_value(value_len);

    for idx in 0..n_keys {
        key_values.push((idx, v.clone()));
    }

    key_values
}

pub fn timed_cache_sequential((max_capacity, n_keys, value_len): (usize, usize, usize)) {
    let key_values = gernerate_key_values(n_keys, value_len);

    let mut cache = TimedCache::<usize, usize, String>::new(Some(max_capacity));

    for (k, v) in key_values.iter() {
        cache.insert(k.clone(), v.clone());
    }

    assert!(cache.len() <= max_capacity);

    for (k, _) in key_values.iter() {
        let _ = cache.get(k);
    }
}

pub fn timed_cache_v2_sequential((max_capacity, n_keys, value_len): (usize, usize, usize)) {
    let key_values = gernerate_key_values(n_keys, value_len);

    let mut cache = TimedCacheV2::<usize, usize, String>::new(Some(max_capacity));

    for (k, v) in key_values.iter() {
        cache.insert(k.clone(), v.clone());
    }
    assert!(cache.len() <= max_capacity);

    for (k, _) in key_values.iter() {
        let _ = cache.get(k);
    }
}

fn insert_and_get<R>(
    cache: Arc<ConcurrentCache<usize, R, usize, String>>,
    n_keys: usize,
    value_len: usize,
) where
    R: KeyRegistry<usize, KeyExtension = usize> + GetKey<usize>,
{
    let key_values = gernerate_key_values(n_keys, value_len);

    for (k, v) in key_values.iter() {
        cache.insert(k.clone(), v.clone());
    }

    for (k, _) in key_values.iter() {
        let _ = cache.get(k);
    }
}

pub fn timed_cache_parallel((max_capacity, n_keys, value_len): (usize, usize, usize)) {
    let cache = Arc::new(ConcurrentTimedCache::<usize, usize, String>::new(Some(
        max_capacity,
    )));

    let mut handles = Vec::new();
    for _ in 0..4 {
        let thread_cache = cache.clone();
        handles.push(std::thread::spawn(move || {
            insert_and_get(thread_cache, n_keys, value_len)
        }));
    }

    assert!(cache.len() <= max_capacity);
}

pub fn timed_cache_v2_parallel((max_capacity, n_keys, value_len): (usize, usize, usize)) {
    let cache = Arc::new(ConcurrentTimedCacheV2::<usize, usize, String>::new(Some(
        max_capacity,
    )));

    let mut handles = Vec::new();
    for _ in 0..4 {
        let thread_cache = cache.clone();
        handles.push(std::thread::spawn(move || {
            insert_and_get(thread_cache, n_keys, value_len)
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    assert!(cache.len() <= max_capacity);
}
