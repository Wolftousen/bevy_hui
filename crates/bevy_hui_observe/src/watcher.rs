#![allow(dead_code)]

use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct HuiPropertyWatchers(HashMap<String, Vec<Entity>>);

impl HuiPropertyWatchers {
    pub fn register_property(&mut self, key: impl Into<String>) {
        let key = key.into();
        match self.0.get_mut(&key) {
            Some(_) => {}
            None => {
                self.0.insert(key, Vec::new());
            }
        }
    }

    pub fn register_entity(&mut self, key: impl Into<String>, value: Entity) {
        let key = key.into();

        match self.0.get_mut(&key) {
            Some(v) => {
                v.push(value)
            }
            None => {}
        }
    }

    pub fn get(&self, key: impl Into<String>) -> Option<&Vec<Entity>> {
        self.0.get(&key.into())
    }
}