use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiBackgroundColor;

impl HuiObservationEvent<HuiBackgroundColor> {
    pub fn new(value: Color) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Color(value.clone()),
        }
    }
}

pub fn observe_background_color(
    trigger: Trigger<HuiObservationEvent<HuiBackgroundColor>>,
    mut query: Query<&mut BackgroundColor>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::Color(s) => s,
        _ => return,
    };

    comp.0 = value.clone();
}
