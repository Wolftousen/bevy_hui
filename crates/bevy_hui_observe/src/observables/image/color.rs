use bevy::prelude::*;
use crate::prelude::HuiObservableType;

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

impl HuiObservableType for HuiImageColor {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_image_color);
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