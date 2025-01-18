use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiImageColor;

impl HuiObservationEvent<HuiImageColor> {
    pub fn new(value: Color) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Color(value),
        }
    }
}

pub fn observe_image_color(
    trigger: Trigger<HuiObservationEvent<HuiImageColor>>,
    mut query: Query<&mut ImageNode>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::Color(s) => s,
        _ => return,
    };

    comp.color = value.clone();
}