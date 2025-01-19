use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiOverflowClipMargin;

impl HuiObservationEvent<HuiOverflowClipMargin> {
    pub fn new(value: f32) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::F32(value.clone()),
        }
    }
}

impl HuiObservableType for HuiOverflowClipMargin {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_overflow_clip_margin);
    }
}

pub fn observe_overflow_clip_margin(
    trigger: Trigger<HuiObservationEvent<HuiOverflowClipMargin>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::F32(s) => s,
        _ => return,
    };

    comp.overflow_clip_margin.margin = value.clone();
}
