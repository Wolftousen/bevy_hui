use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiText;

impl HuiObservationEvent<HuiText> {
    pub fn new(value: impl Into<String>) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::String(value.into()),
        }
    }
}

pub fn observe_text(
    trigger: Trigger<HuiObservationEvent<HuiText>>,
    mut query: Query<&mut Text>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::String(s) => s,
        _ => return,
    };

    comp.0 = value.clone();
}