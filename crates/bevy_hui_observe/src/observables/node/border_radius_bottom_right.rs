use bevy::prelude::*;
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiBorderRadiusBottomRight;

impl HuiObservationEvent<HuiBorderRadiusBottomRight> {
    pub fn new(value: Val) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Val(value.clone()),
        }
    }
}

impl HuiObservableType for HuiBorderRadiusBottomRight {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_border_radius_bottom_right);
    }
}

pub fn observe_border_radius_bottom_right(
    trigger: Trigger<HuiObservationEvent<HuiBorderRadiusBottomRight>>,
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

    comp.bottom_right = value.clone();
}
