use bevy::{prelude::*, ui::widget::NodeImageMode};
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiImageMode;

impl HuiObservationEvent<HuiImageMode> {
    pub fn new(value: NodeImageMode) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::ImageMode(value),
        }
    }
}

impl HuiObservableType for HuiImageMode {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_image_mode);
    }
}

pub fn observe_image_mode(
    trigger: Trigger<HuiObservationEvent<HuiImageMode>>,
    mut query: Query<&mut ImageNode>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::ImageMode(s) => s,
        _ => return,
    };

    comp.image_mode = value.clone();
}