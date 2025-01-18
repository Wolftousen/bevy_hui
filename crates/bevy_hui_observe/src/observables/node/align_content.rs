use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiAlignContent;

impl HuiObservationEvent<HuiAlignContent> {
    pub fn new(value: AlignContent) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::AlignContent(value.clone()),
        }
    }
}

pub fn observe_align_content(
    trigger: Trigger<HuiObservationEvent<HuiAlignContent>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::AlignContent(s) => s,
        _ => return,
    };

    comp.align_content = value.clone();
}
