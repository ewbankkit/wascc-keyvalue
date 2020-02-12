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
}
