#[macro_use]
extern crate log;

use lru_simple::LruCache1;
use lru_simple::Lru;
use std::hash::Hash;
use std::fmt::Debug;

fn main() {
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();

    let mut cache = Cache::new(3);
    cache.put(1, "one");
    cache.put(2, "two");
    cache.put(3, "three");
    cache.put(4, "four");
}


struct Cache<K: Hash + Eq + Clone + Debug + 'static, V: Debug + 'static> {
    cache: Box<dyn Lru<K, V>>
}

impl<K: Hash + Eq + Clone + Debug + 'static, V: Debug + 'static> Cache<K, V> {
    pub fn new(cap: usize) -> Self {
        Cache {
            cache: Box::new(LruCache1::new(cap)),
        }
    }

    pub fn put(&mut self, k: K, v: V) -> Option<V> {
        self.cache.put(k, v)
    }

    pub fn get(&mut self, k: &K) -> Option<&V> {
        self.cache.get(k)
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}