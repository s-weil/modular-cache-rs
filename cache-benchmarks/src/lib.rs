// generate data
extern crate modular_cache;

use modular_cache::timed_cache::{TimedCache, TimedCacheV2};
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

    let mut cache = TimedCache::<usize, String>::new(Some(max_capacity));

    for (k, v) in key_values.iter() {
        cache.insert(k.clone(), v.clone());
    }

    assert!(cache.len() <= max_capacity);

    for (k, v) in key_values.iter() {
        let v_c = cache.get(k);
        // assert!(v_c.is_some());
        // assert_eq!(v_c.unwrap(), v);
    }
}

pub fn timed_cache_v2_sequential((max_capacity, n_keys, value_len): (usize, usize, usize)) {
    let key_values = gernerate_key_values(n_keys, value_len);

    let mut cache = TimedCacheV2::<usize, String>::new(Some(max_capacity));

    for (k, v) in key_values.iter() {
        cache.insert(k.clone(), v.clone());
    }
    assert!(cache.len() <= max_capacity);

    for (k, v) in key_values.iter() {
        let v_c = cache.get(k);
        // assert!(v_c.is_some());
        // assert_eq!(v_c.unwrap(), v);
    }
}
