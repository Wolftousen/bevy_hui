use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiFlipY;

impl HuiObservationEvent<HuiFlipY> {
    pub fn new(value: bool) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Bool(value),
        }
    }
}

pub fn observe_flip_y(
    trigger: Trigger<HuiObservationEvent<HuiFlipY>>,
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

    comp.flip_y = value.clone();
}