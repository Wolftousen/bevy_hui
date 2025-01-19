use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiFont;

impl HuiObservationEvent<HuiFont> {
    pub fn new(value: impl Into<String>) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::String(value.into()),
        }
    }
}

impl HuiObservableType for HuiFont {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_font);
    }
}

pub fn observe_font(
    trigger: Trigger<HuiObservationEvent<HuiFont>>,
    mut query: Query<&mut TextFont>,
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

    comp.font = asset_server.load(value);
}