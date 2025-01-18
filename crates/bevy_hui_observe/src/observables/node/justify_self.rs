use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiJustifySelf;

impl HuiObservationEvent<HuiJustifySelf> {
    pub fn new(value: JustifySelf) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::JustifySelf(value.clone()),
        }
    }
}

pub fn observe_justify_self(
    trigger: Trigger<HuiObservationEvent<HuiJustifySelf>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::JustifySelf(s) => s,
        _ => return,
    };

    comp.justify_self = value.clone();
}
