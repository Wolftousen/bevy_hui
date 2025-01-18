use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiAlignSelf;

impl HuiObservationEvent<HuiAlignSelf> {
    pub fn new(value: AlignSelf) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::AlignSelf(value.clone()),
        }
    }
}

pub fn observe_align_self(
    trigger: Trigger<HuiObservationEvent<HuiAlignSelf>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::AlignSelf(s) => s,
        _ => return,
    };

    comp.align_self = value.clone();
}
