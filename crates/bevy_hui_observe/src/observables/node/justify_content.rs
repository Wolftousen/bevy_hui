use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiJustifyContent;

impl HuiObservationEvent<HuiJustifyContent> {
    pub fn new(value: JustifyContent) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::JustifyContent(value.clone()),
        }
    }
}

impl HuiObservableType for HuiJustifyContent {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_justify_content);
    }
}

pub fn observe_justify_content(
    trigger: Trigger<HuiObservationEvent<HuiJustifyContent>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::JustifyContent(s) => s,
        _ => return,
    };

    comp.justify_content = value.clone();
}
