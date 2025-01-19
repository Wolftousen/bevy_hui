use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiJustifyItems;

impl HuiObservationEvent<HuiJustifyItems> {
    pub fn new(value: JustifyItems) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::JustifyItems(value.clone()),
        }
    }
}

impl HuiObservableType for HuiJustifyItems {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_justify_items);
    }
}

pub fn observe_justify_items(
    trigger: Trigger<HuiObservationEvent<HuiJustifyItems>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::JustifyItems(s) => s,
        _ => return,
    };

    comp.justify_items = value.clone();
}
