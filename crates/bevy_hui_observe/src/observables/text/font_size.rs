use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiFontSize;

impl HuiObservationEvent<HuiFontSize> {
    pub fn new(value: f32) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::F32(value),
        }
    }
}

pub fn observe_font_size(
    trigger: Trigger<HuiObservationEvent<HuiFontSize>>,
    mut query: Query<&mut TextFont>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::F32(s) => s,
        _ => return,
    };

    comp.font_size = value.clone();
}