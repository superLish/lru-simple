// use std::collections::HashMap;
// 双向链表实现思路： 每当新插入或者访问数据，将数据放到链表头部，当数据满了的时候，从链表尾部删除数据。
// 哈希表+双向链表实现   主要思路是在双向链表的实现基础上，增加一个哈希表，用于索引key，使得插入和删除的复杂度降低到O(1)。
// 具体一点就是，哈希表的K可以认为是一个索引，V 是一个双向链表，每个V都有前驱节点和后驱节点的指针。



use std::mem::MaybeUninit;

use std::hash::Hash;
use std::collections::HashMap;
use crate::Lru;
use std::borrow::BorrowMut;



struct Entry<K: Hash + Eq + Clone, V> {
    k: MaybeUninit<K>,
    v: MaybeUninit<V>,
    prev: *mut Entry<K, V>,
    next: *mut Entry<K, V>,
}

impl<K: Hash + Eq + Clone, V> Entry<K, V> {
    pub fn new(k: K, v: V) -> Self {
        Self {
            k: MaybeUninit::new(k),
            v: MaybeUninit::new(v),
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        }
    }

    pub fn new_uninit() -> Self {
        Self {
            k: MaybeUninit::uninit(),
            v: MaybeUninit::uninit(),
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        }
    }

}


pub struct LruCache2<K: Hash + Eq + Clone, V> {
    map: HashMap<K, Box<Entry<K, V>>>,
    capacity: usize,
    head: *mut Entry<K, V>,         // 最近使用的， 新增或者访问节点的时候，节点要前移到head
    tail: *mut Entry<K, V>,         // 最近最少使用的， 当capacity满了的时候，要从尾部节点删除
}

impl<K: Hash + Eq + Clone, V> LruCache2<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        let mut cache = Self {
            map: HashMap::with_capacity(capacity),
            capacity,
            head: Box::into_raw(Box::new(Entry::new_uninit())),
            tail: Box::into_raw(Box::new(Entry::new_uninit())),
        };

        unsafe {
            (*cache.head).next = cache.tail;
            (*cache.tail).prev = cache.head;
        }

        cache
    }
}


impl<K: Hash + Eq + Clone, V> Lru<K, V> for LruCache2<K, V> {
    // 如果是新插入，直接插入到head前面，更新head；如果是更新已有值，则变更指针，使得原有值为head前面，
    fn put(&mut self, k: K, v: V) -> Option<V> {
    /*
        // 如果是更新已有值，
        if let Some(entry) = self.map.get_mut(&k) {
            // 原有位置删除
            let next_entry = entry.next;
            let prev_entry = entry.prev;
            unsafe {
                (*next_entry).prev = entry.prev;
                (*prev_entry).next = entry.next;
            }

            // 放到head前面
            entry.next = self.head;
            entry.prev = std::ptr::null_mut();

            // 更新值
            let old = std::mem::replace(entry.v.borrow_mut(), v);
            return Some(old);
        }

        // 插入新值
        let entry = Entry::new(k.clone(), v, std::ptr::null_mut(), self.head.clone());
        let new_head = Box::new(entry);
        let new_ptr = Box::into_raw(new_head);
        let new_head_clone = unsafe {
            Box::from_raw(new_ptr.clone())
        };
        self.map.insert(k.clone(), new_head_clone);

        // 更改旧head的前驱节点
        unsafe {
            (*self.head).prev = new_ptr.clone();
        }

        // 更新head
    */
        None
    }

    fn get(&mut self, k: &K) -> Option<&V> {
        // get后，如果key存在，则将key移到head后第一个节点
        if let Some(entry) = self.map.get_mut(k) {
            let ref value = entry.v;
            let value_unwrap = unsafe {
                &*value.as_ptr()
            };
            // 如果已经是唯一头部，则不进行任何操作
            if self.head == entry.prev {
                return Some(value_unwrap);
            }

            let current_ptr = unsafe {(*entry.prev).next};

            // 原有位置删除
            unsafe {
                (*entry.prev).next = entry.next;
                (*entry.next).prev = entry.prev;
            }

            // 放到head后面第一个节点的位置
            // unsafe {
            //     (*self.head).next.prev = current_ptr;
            //     current_ptr.next = self.head.next;
            //     current_ptr.prev = self.head;
            //     self.head.next = current_ptr;
            // }

            // // 更新head

            return Some(value_unwrap);
        }

        None
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn clear(&mut self) {
        self.map.clear();
        unsafe {
            (*self.head).next = self.tail;
            (*self.tail).prev = self.head;
        }
    }
}