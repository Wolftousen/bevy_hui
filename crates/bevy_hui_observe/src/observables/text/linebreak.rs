use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiLineBreak;

impl HuiObservationEvent<HuiLineBreak> {
    pub fn new(value: LineBreak) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::LineBreak(value.into()),
        }
    }
}

impl HuiObservableType for HuiLineBreak {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_linebreak);
    }
}

pub fn observe_linebreak(
    trigger: Trigger<HuiObservationEvent<HuiLineBreak>>,
    mut query: Query<&mut TextLayout>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::LineBreak(s) => s,
        _ => return,
    };

    comp.linebreak = value.clone();
}