use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiAlignContent;

impl HuiObservationEvent<HuiAlignContent> {
    pub fn new(value: AlignContent) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::AlignContent(value.clone()),
        }
    }
}

impl HuiObservableType for HuiAlignContent {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_align_content);
    }
}

pub fn observe_align_content(
    trigger: Trigger<HuiObservationEvent<HuiAlignContent>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::AlignContent(s) => s,
        _ => return,
    };

    comp.align_content = value.clone();
}
