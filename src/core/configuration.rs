use std::{collections::HashMap, sync::Arc};

use crate::core::input::processors::Field;

#[derive(Default, Debug, Clone)]
pub struct Configuration {
    current: HashMap<String, Field>,
    parent: Option<Arc<Configuration>>,
}
impl Configuration {
    pub fn new(map: HashMap<String, Field>) -> Self {
        Self {
            current: map,
            parent: None,
        }
    }

    pub fn with_parent(map: HashMap<String, Field>, parent: Option<Arc<Configuration>>) -> Self {
        Self {
            current: map,
            parent,
        }
    }

    pub fn get(&self, key: &str) -> Option<&Field> {
        // Check current level
        if let Some(value) = self.current.get(key) {
            return Some(value);
        }

        // Recursively check parent
        self.parent.as_ref().and_then(|p| p.get(key))
    }

    // pub fn get_string(&self, key: &str) -> Option<String> {
    //     self.get(key).and_then(|field| {
    //         if let Field::Leaf(value) = field {
    //             Some(value.clone())
    //         } else {
    //             None
    //         }
    //     })
    // }

    pub fn insert(&mut self, key: String, value: Field) {
        self.current.insert(key, value);
    }
}
