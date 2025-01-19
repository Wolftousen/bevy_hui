use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiZIndex;

impl HuiObservationEvent<HuiZIndex> {
    pub fn new(value: i32) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::I32(value.clone()),
        }
    }
}

impl HuiObservableType for HuiZIndex {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_z_index);
    }
}

pub fn observe_z_index(
    trigger: Trigger<HuiObservationEvent<HuiZIndex>>,
    mut query: Query<&mut ZIndex>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::I32(s) => s,
        _ => return,
    };

    comp.0 = value.clone();
}
