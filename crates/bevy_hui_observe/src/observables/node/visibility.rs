use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiVisibility;

impl HuiObservationEvent<HuiVisibility> {
    pub fn new(value: Visibility) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Visibility(value.clone()),
        }
    }
}

pub fn observe_visibility(
    trigger: Trigger<HuiObservationEvent<HuiVisibility>>,
    mut commands: Commands,
    query: Query<Entity, With<Visibility>>,
) {
    let Ok(entity) = query.get(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::Visibility(s) => s,
        _ => return,
    };

    commands.entity(entity).insert(value.clone());
}
