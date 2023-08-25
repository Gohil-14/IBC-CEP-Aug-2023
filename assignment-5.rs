use std::collections::HashMap;

// Define the SortByKey trait
trait SortByKey<K, V> {
    fn sort_by_key(&mut self);
}

// Implement the SortByKey trait for HashMap
impl<K: Ord, V> SortByKey<K, V> for HashMap<K, V> {
    fn sort_by_key(&mut self) {
        // Collect the keys from the HashMap and sort them
        let sorted_keys: Vec<_> = self.keys().cloned().collect();
        sorted_keys.sort();

        // Create a new HashMap to store the sorted key-value pairs
        let mut sorted_map = HashMap::new();

        // Populate the sorted map with the sorted key-value pairs
        for key in sorted_keys.iter() {
            if let Some(value) = self.remove(key) {
                sorted_map.insert(key.clone(), value);
            }
        }

        // Replace the original HashMap with the sorted map
        *self = sorted_map;
    }
}

fn main() {
    // Create a new instance of the HashMap struct
    let mut my_map: HashMap<i32, &str> = HashMap::new();

    // Add key-value pairs to the map
    my_map.insert(3, "apple");
    my_map.insert(1, "banana");
    my_map.insert(2, "orange");
    my_map.insert(5, "grape");
    my_map.insert(4, "kiwi");

    println!("Original map: {:?}", my_map);

    // Sort the map by keys using the SortByKey trait
    my_map.sort_by_key();

    println!("Sorted map: {:?}", my_map);
}
