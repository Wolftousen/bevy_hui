use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiOverflowClipBox;

impl HuiObservationEvent<HuiOverflowClipBox> {
    pub fn new(value: OverflowClipBox) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::OverflowClipBox(value.clone()),
        }
    }
}

pub fn observe_overflow_clip_box(
    trigger: Trigger<HuiObservationEvent<HuiOverflowClipBox>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::OverflowClipBox(s) => s,
        _ => return,
    };

    comp.overflow_clip_margin.visual_box = value.clone();
}
