use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiBorderRadiusBottomLeft;

impl HuiObservationEvent<HuiBorderRadiusBottomLeft> {
    pub fn new(value: Val) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Val(value.clone()),
        }
    }
}

impl HuiObservableType for HuiBorderRadiusBottomLeft {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_border_radius_bottom_left);
    }
}

pub fn observe_border_radius_bottom_left(
    trigger: Trigger<HuiObservationEvent<HuiBorderRadiusBottomLeft>>,
    mut query: Query<&mut BorderRadius>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::Val(s) => s,
        _ => return,
    };

    comp.bottom_left = value.clone();
}
