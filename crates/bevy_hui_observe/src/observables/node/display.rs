use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiDisplay;

impl HuiObservationEvent<HuiDisplay> {
    pub fn new(value: Display) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Display(value.clone()),
        }
    }
}

pub fn observe_display(
    trigger: Trigger<HuiObservationEvent<HuiDisplay>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::Display(s) => s,
        _ => return,
    };

    comp.display = value.clone();
}
