use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiScrollPositionX;

impl HuiObservationEvent<HuiScrollPositionX> {
    pub fn new(value: f32) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::F32(value),
        }
    }
}

pub fn observe_scroll_position_x(
    trigger: Trigger<HuiObservationEvent<HuiScrollPositionX>>,
    mut query: Query<&mut ScrollPosition>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::F32(s) => s,
        _ => return,
    };

    comp.offset_x = value.clone();
}