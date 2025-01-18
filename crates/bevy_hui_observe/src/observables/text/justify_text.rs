use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiJustifyText;

impl HuiObservationEvent<HuiJustifyText> {
    pub fn new(value: JustifyText) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::JustifyText(value.into()),
        }
    }
}

pub fn observe_justify_text(
    trigger: Trigger<HuiObservationEvent<HuiJustifyText>>,
    mut query: Query<&mut TextLayout>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::JustifyText(s) => s,
        _ => return,
    };

    comp.justify = value.clone();
}