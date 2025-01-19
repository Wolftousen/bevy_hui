use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiAspectRatio;

impl HuiObservationEvent<HuiAspectRatio> {
    pub fn new(value: Option<f32>) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::OptionF32(value.clone()),
        }
    }
}

impl HuiObservableType for HuiAspectRatio {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_aspect_ratio);
    }
}

pub fn observe_aspect_ratio(
    trigger: Trigger<HuiObservationEvent<HuiAspectRatio>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::OptionF32(s) => s,
        _ => return,
    };

    comp.aspect_ratio = value.clone();
}
