use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::result::Result;

pub enum KeyValueItem {
    Atomic(i32),
    Scalar(String),
    List(Vec<String>),
    Set(HashSet<String>),
}

pub struct KeyValueStore {
    items: HashMap<String, KeyValueItem>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore {
            items: HashMap::new(),
        }
    }

    // Retrieves the value for a given key, returning the value and whether or not it existed.
    pub fn get(&self, key: &str) -> Result<String, Box<dyn Error>> {
        self.items.get(key).map_or_else(
            || Err("No such key".into()),
            |v| {
                if let KeyValueItem::Scalar(ref s) = v {
                    Ok(s.clone())
                } else {
                    Err("Attempt to fetch non-scalar".into())
                }
            },
        )
    }

    // Sets the value for a given key
    pub fn set(&mut self, key: &str, value: String) -> Result<(), Box<dyn Error>> {
        self.items
        .entry(key.to_string())
        .and_modify(|v| {
            if let KeyValueItem::Scalar(_) = v {
                *v = KeyValueItem::Scalar(value.clone())
            }
        })
        .or_insert(KeyValueItem::Scalar(value));

        Ok(())
    }

    // Performs an atomic add operation, returning the new value
    pub fn atomic_add(&self, key: &str, value: i32) ->  Result<i32, Box<dyn Error>> {
        Ok(42)
    }

    // Adds a string value to a list stored within a given key.
    pub fn list_add(&self, key: &str, item: &str) ->  Result<usize, Box<dyn Error>> {
        Ok(42)
    }

    // Deletes all occurrences of an item in a list.
    pub fn list_del_item(&self, key: &str, item: &str) -> Result<usize, Box<dyn Error>> {
        Ok(42)
    }

    // Deletes the given key.
    pub fn del_key(&self, key: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    // Requests a list of values contained within a given key.
    pub fn list_range(&self, key: &str, start: isize, stop_include: isize)-> Result<Vec<String>, Box<dyn Error>> {
        Ok(vec!["no".into()])
    }

    // Clears a list.
    pub fn list_clear(&self, key: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    // Adds an item to a set.
    pub fn set_add(&self, key: &str, value: &str)  -> Result<usize, Box<dyn Error>> {
        Ok(42)
    }

    // Removes an item from a set.
    pub fn set_remove(&self, key: &str, value: &str) -> Result<usize, Box<dyn Error>> {
        Ok(42)
    }

    // Returns the union of sets indicated by list of keys.
    pub fn set_union(&self, keys: Vec<String>)-> Result<Vec<String>, Box<dyn Error>> {
        Ok(Vec::new())
    }

    // Returns the intersection of all sets indicated by the list of keys.
    pub fn set_intersect(&self, keys: Vec<String>)-> Result<Vec<String>, Box<dyn Error>> {
        Ok(Vec::new())
    }

    // Returns all members of a given set.
    pub fn set_members(&self, key: &str)-> Result<Vec<String>, Box<dyn Error>> {
        Ok(Vec::new())
    }

    // Indicates whether or not a given key exists in the data store.
    pub fn exists(&self, key: &str)-> Result<bool, Box<dyn Error>> {
        Ok(false)
    }
}
