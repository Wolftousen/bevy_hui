use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiText;

impl HuiObservationEvent<HuiText> {
    pub fn new(value: impl Into<String>) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::String(value.into()),
        }
    }
}

impl HuiObservableType for HuiText {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_text);
    }
}

pub fn observe_text(
    trigger: Trigger<HuiObservationEvent<HuiText>>,
    mut query: Query<&mut Text>,
) {
    println!("observing text");
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        println!("failed to get text component");
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::String(s) => s,
        _ => return,
    };

    comp.0 = value.clone();
}