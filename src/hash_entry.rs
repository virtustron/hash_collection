use std::ptr;

pub struct HashEntry<K, V> {
    key: K,
    value: V,
    next: *mut HashEntry<K, V>,
}

impl<K, V> HashEntry<K, V> {
    pub fn new(key: K, value: V) -> HashEntry<K, V> {
        let next: *mut HashEntry<K, V> =  ptr::null_mut() as *mut HashEntry<K, V>;
        
        HashEntry {
            key,
            value,
            next,
        }
    }

    pub fn get_next(self) -> *mut HashEntry<K, V> {
        return self.next;
    }

    pub fn get_key(self) -> K {
        return self.key;
    }

    pub fn get_value(self) -> V {
        return self.value;
    }
}