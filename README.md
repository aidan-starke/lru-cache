# LRU Cache

Generally cache is some data structure used to store some results of an expensive computation.
Cache's have fixed memory limits and therefore implement policies about when items should be evicted.

A Least-Recently-Used eviction policy drops the least recently accessed item when the cache is at capacity.
It is economical for trending request patterns that move in a time window e.g. today's top articles, the latest 10 blocks on explorer etc. where requests are common within some time frame and change slowly, ensuring good hit rate.

General implementation goals:
- Only the N most recently requested items are kept in the cache i.e memory usage of the cache is fixed at O(N)

Rust specific implementation goals:
- generic over key and value types
- safe for multi-thread usage using tokio

```rust
/// TODO: implement this trait
trait LRUCache<K, V> {
    /// Initialize the cache with `max` items stored
    fn initialize(max: usize) -> Self;
    /// Retrieve the value stored by key, if it exists
    fn get(&self, key: K) -> Option<V>;
    /// Store a `value` under `key`
    fn set(&mut self, key: K, value: V);
}
```

## 
- write your own unit tests
- use the same test harness at the end performance stuff