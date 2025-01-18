#![allow(dead_code)]

use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct HuiPropertyWatchers(HashMap<String, Vec<Entity>>);

impl HuiPropertyWatchers {
    fn register(&mut self, key: impl Into<String>, value: Entity) {
        let key = key.into();

        match self.0.get_mut(&key) {
            Some(v) => v.push(value),
            None => {
                self.0.insert(key, vec![value]);
            }
        }
    }
}