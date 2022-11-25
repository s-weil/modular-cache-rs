pub mod cache;
pub mod timed_cache;

/*
TODO:
    - expiration policies
    - lru impplementation
    - concurrent (with std/parkinglot/tokio) via feature
    - test different timed cache: split into lookup and ordered list
    - rescale capacity

*/
