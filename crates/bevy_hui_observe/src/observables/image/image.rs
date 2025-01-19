use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiImage;

impl HuiObservationEvent<HuiImage> {
    pub fn new(value: impl Into<String>) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::String(value.into()),
        }
    }
}

impl HuiObservableType for HuiImage {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_image);
    }
}

pub fn observe_image(
    trigger: Trigger<HuiObservationEvent<HuiImage>>,
    mut query: Query<&mut ImageNode>,
    asset_server: Res<AssetServer>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::String(s) => s,
        _ => return,
    };

    comp.image = asset_server.load(value);
}