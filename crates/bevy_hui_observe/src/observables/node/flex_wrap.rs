use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiFlexWrap;

impl HuiObservationEvent<HuiFlexWrap> {
    pub fn new(value: FlexWrap) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::FlexWrap(value.clone()),
        }
    }
}

impl HuiObservableType for HuiFlexWrap {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_flex_wrap);
    }
}

pub fn observe_flex_wrap(
    trigger: Trigger<HuiObservationEvent<HuiFlexWrap>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::FlexWrap(s) => s,
        _ => return,
    };

    comp.flex_wrap = value.clone();
}
