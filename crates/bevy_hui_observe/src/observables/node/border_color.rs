use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiBorderColor;

impl HuiObservationEvent<HuiBorderColor> {
    pub fn new(value: Color) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Color(value.clone()),
        }
    }
}

pub fn observe_border_color(
    trigger: Trigger<HuiObservationEvent<HuiBorderColor>>,
    mut query: Query<&mut BorderColor>,
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
