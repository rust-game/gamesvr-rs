use std::collections::HashMap;
use std::hash::Hash;
use handle::Handle;

/// route
pub trait Route<K, V> where K: Hash, V: Handle {
    fn set(&mut self, K, V) {}
    fn get(&self, K) -> Option<V> {
        Ok(V)
    }
}

/// default impl
pub struct Router<K, V> where V: Handle {
    inner: HashMap<K, V>,
}

impl Router<K, V> {
    fn new() {
        Router {
            inner: HashMap::new()
        }
    }
}


impl Route<K, V> for Router<K, V> {
    fn set(&mut self, k: K, v: V) {
        self.inner.insert(k, v)
    }

    fn get(&self, k: K) {
        self.inner.get(&k)
    }
}
