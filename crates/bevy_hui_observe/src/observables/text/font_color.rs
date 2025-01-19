use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiFontColor;

impl HuiObservationEvent<HuiFontColor> {
    pub fn new(value: Color) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Color(value),
        }
    }
}

impl HuiObservableType for HuiFontColor {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_font_color);
    }
}

pub fn observe_font_color(
    trigger: Trigger<HuiObservationEvent<HuiFontColor>>,
    mut query: Query<&mut TextColor>,
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