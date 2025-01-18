use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiFlexShrink;

impl HuiObservationEvent<HuiFlexShrink> {
    pub fn new(value: f32) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::F32(value.clone()),
        }
    }
}

pub fn observe_flex_shrink(
    trigger: Trigger<HuiObservationEvent<HuiFlexShrink>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::F32(s) => s,
        _ => return,
    };

    comp.flex_shrink = value.clone();
}
