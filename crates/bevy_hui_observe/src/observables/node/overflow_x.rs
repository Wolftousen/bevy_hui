use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiOverflowX;

impl HuiObservationEvent<HuiOverflowX> {
    pub fn new(value: Overflow) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Overflow(value.clone()),
        }
    }
}

pub fn observe_overflow_x(
    trigger: Trigger<HuiObservationEvent<HuiOverflowX>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::Overflow(s) => s,
        _ => return,
    };

    comp.overflow = value.clone();
}
