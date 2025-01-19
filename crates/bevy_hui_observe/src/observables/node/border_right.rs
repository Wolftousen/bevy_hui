use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiBorderRight;

impl HuiObservationEvent<HuiBorderRight> {
    pub fn new(value: Val) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Val(value.clone()),
        }
    }
}

impl HuiObservableType for HuiBorderRight {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_border_right);
    }
}

pub fn observe_border_right(
    trigger: Trigger<HuiObservationEvent<HuiBorderRight>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::Val(s) => s,
        _ => return,
    };

    comp.border.right = value.clone();
}
