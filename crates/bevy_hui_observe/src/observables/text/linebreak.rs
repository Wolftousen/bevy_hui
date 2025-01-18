use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiLineBreak;

impl HuiObservationEvent<HuiLineBreak> {
    pub fn new(value: LineBreak) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::LineBreak(value.into()),
        }
    }
}

pub fn observe_linebreak(
    trigger: Trigger<HuiObservationEvent<HuiLineBreak>>,
    mut query: Query<&mut TextLayout>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::LineBreak(s) => s,
        _ => return,
    };

    comp.linebreak = value.clone();
}