pub mod cache;
pub mod concurrent_cache;
pub mod key;
pub mod queued_cache;

/*
TODO:
    - split key registry in smaller pieces and try to simplify signature of cache wrt number of generics -> avoid key registry at all!
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
