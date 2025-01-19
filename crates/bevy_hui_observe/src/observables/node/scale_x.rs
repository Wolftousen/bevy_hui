use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiScaleX;

impl HuiObservationEvent<HuiScaleX> {
    pub fn new(value: f32) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::F32(value),
        }
    }
}

impl HuiObservableType for HuiScaleX {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_scale_x);
    }
}

pub fn observe_scale_x(
    trigger: Trigger<HuiObservationEvent<HuiScaleX>>,
    mut query: Query<&mut Transform>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::F32(s) => s,
        _ => return,
    };

    comp.scale[0] = value.clone();
}