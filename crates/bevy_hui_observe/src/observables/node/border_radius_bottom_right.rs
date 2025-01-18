use bevy::prelude::*;
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
