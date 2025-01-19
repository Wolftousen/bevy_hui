use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiFlipX;

impl HuiObservationEvent<HuiFlipX> {
    pub fn new(value: bool) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Bool(value),
        }
    }
}

impl HuiObservableType for HuiFlipX {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_flip_x);
    }
}

pub fn observe_flip_x(
    trigger: Trigger<HuiObservationEvent<HuiFlipX>>,
    mut query: Query<&mut ImageNode>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::Bool(s) => s,
        _ => return,
    };

    comp.flip_x = value.clone();
}