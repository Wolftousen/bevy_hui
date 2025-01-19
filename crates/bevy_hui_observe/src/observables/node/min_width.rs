use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiMinWidth;

impl HuiObservationEvent<HuiMinWidth> {
    pub fn new(value: Val) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Val(value.clone()),
        }
    }
}

impl HuiObservableType for HuiMinWidth {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_min_width);
    }
}

pub fn observe_min_width(
    trigger: Trigger<HuiObservationEvent<HuiMinWidth>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::Val(s) => s,
        _ => return,
    };

    comp.min_width = value.clone();
}
