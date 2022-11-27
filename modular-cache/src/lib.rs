pub mod cache;
pub mod concurrent_cache;
pub mod key;
pub mod timed_cache;

/*
TODO:
    - expiration policies
    - lru impplementation
    - simplest keyregistry with hashmap! -> doesnt make sense
    - concurrent (with std/parkinglot/tokio) via feature
    - rescale capacity
    - key invalidation
    - generalize with randomstate and buildhasher
    - housekeeping
    - benchmark project. check insertion, getting, inlined, mutex vs rwlock etc etc; check also external implementations
*/
