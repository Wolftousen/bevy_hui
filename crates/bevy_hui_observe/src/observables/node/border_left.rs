use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiBorderLeft;

impl HuiObservationEvent<HuiBorderLeft> {
    pub fn new(value: Val) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Val(value.clone()),
        }
    }
}

pub fn observe_border_left(
    trigger: Trigger<HuiObservationEvent<HuiBorderLeft>>,
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

    comp.border.left = value.clone();
}
