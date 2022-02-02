use std::ptr;

pub struct HashEntry<K, V> {
    key: K,
    value: V,
    next: *mut HashEntry<K, V>,
}

impl<K, V> HashEntry<K, V> {
    fn new(key: K, value: V) -> HashEntry<K, V> {
        let next: *mut HashEntry<K, V> =  ptr::null_mut() as *mut HashEntry<K, V>;
        
        HashEntry {
            key,
            value,
            next,
        }
    }

    fn get_next(self) -> *mut HashEntry<K, V> {
        return self.next;
    }

    fn get_value(self) -> V {
        return self.value;
    }
}