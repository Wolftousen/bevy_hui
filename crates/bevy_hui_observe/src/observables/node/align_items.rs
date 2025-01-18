use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiAlignItems;

impl HuiObservationEvent<HuiAlignItems> {
    pub fn new(value: AlignItems) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::AlignItems(value.clone()),
        }
    }
}

pub fn observe_align_items(
    trigger: Trigger<HuiObservationEvent<HuiAlignItems>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::AlignItems(s) => s,
        _ => return,
    };

    comp.align_items = value.clone();
}
