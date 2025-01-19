use bevy::{prelude::*, text::FontSmoothing};
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiFontSmoothing;

impl HuiObservationEvent<HuiFontSmoothing> {
    pub fn new(value: FontSmoothing) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::FontSmoothing(value.into()),
        }
    }
}

impl HuiObservableType for HuiFontSmoothing {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_font_smoothing);
    }
}

pub fn observe_font_smoothing(
    trigger: Trigger<HuiObservationEvent<HuiFontSmoothing>>,
    mut query: Query<&mut TextFont>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::FontSmoothing(s) => s,
        _ => return,
    };

    comp.font_smoothing = value.clone();
}