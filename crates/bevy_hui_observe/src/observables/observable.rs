#![allow(dead_code)]

use bevy::{ecs::system::IntoObserverSystem, prelude::*, utils::HashMap};

use super::HuiObservableType;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct HuiObservableProperties(HashMap<String, Box<dyn HuiObservableType>>);

impl HuiObservableProperties {
    fn register<E: Event, B: Bundle, M, I: IntoObserverSystem<E, B, M>>(&mut self, key: impl Into<String>, system: impl IntoObserverSystem<E, B, M>)
    where
        E: Event + 'static,
        B: Bundle,
    {
        self.insert(key.into(), Box::new(IntoObserverSystem::into_system(system)));
    }

    fn observe(&mut self, key: impl Into<String>, entity_builder: &mut EntityCommands) {
        if let Some(system) = self.get(&key.into()) {
            entity_builder.observe(system);
        }
    }
}