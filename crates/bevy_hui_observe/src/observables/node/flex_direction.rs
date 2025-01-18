use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiFlexDirection;

impl HuiObservationEvent<HuiFlexDirection> {
    pub fn new(value: FlexDirection) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::FlexDirection(value.clone()),
        }
    }
}

pub fn observe_flex_direction(
    trigger: Trigger<HuiObservationEvent<HuiFlexDirection>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::FlexDirection(s) => s,
        _ => return,
    };

    comp.flex_direction = value.clone();
}
