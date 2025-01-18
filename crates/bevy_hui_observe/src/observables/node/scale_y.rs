use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiScaleY;

impl HuiObservationEvent<HuiScaleY> {
    pub fn new(value: f32) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::F32(value),
        }
    }
}

pub fn observe_scale_y(
    trigger: Trigger<HuiObservationEvent<HuiScaleY>>,
    mut query: Query<&mut Transform>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::F32(s) => s,
        _ => return,
    };

    comp.scale[1] = value.clone();
}