use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiBorderRadiusTopLeft;

impl HuiObservationEvent<HuiBorderRadiusTopLeft> {
    pub fn new(value: Val) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::Val(value.clone()),
        }
    }
}

pub fn observe_border_radius_top_left(
    trigger: Trigger<HuiObservationEvent<HuiBorderRadiusTopLeft>>,
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

    comp.top_left = value.clone();
}
