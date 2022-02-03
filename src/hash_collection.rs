use std::sync::atomic::AtomicPtr;


use crate::hash_entry::HashEntry;


struct HashCollection<K, V> {
    entries: AtomicPtr<HashEntry<K, V>>,      // *mut HashEntry<K, V>
    capacity: usize,

}


impl<K, V> HashCollection<K, V> {
    pub fn do_hashing(self, key: K) -> usize {
        let index: usize;                       

        // TODO implement restriction to type "K" - for "mod" operation 
        index = 0;
        //index = key % self.capacity;

        index
    }


    pub fn new(capacity: usize) -> HashCollection<K, V> {
        let temp_buffer: Vec<HashEntry<K, V>> = Vec::with_capacity(capacity);
        let (ptr, _len, _cap) = temp_buffer.into_raw_parts();
        let temp_buffer = ptr as *mut HashEntry<K, V>;
        let entries = AtomicPtr::new(temp_buffer);

        HashCollection {
            entries,
            capacity,
        }
    }


    pub fn insert(self, key: K, value: V) {

    }

    pub fn get_entry(self, key: K) -> HashEntry<K, V> {
        let index: usize;
        index = self.do_hashing(key);

        // TODO
        // 1. Get entry by index
        // 2. WHILE (entry.key != key) get next entry and check it

    }




} 


#[cfg(test)]
mod tests {
    use crate::hash_collection::HashCollection;

    #[test]
    fn create_hash_collection() {
        let hc: HashCollection<u32, u32> = HashCollection::new(5);
    }

}