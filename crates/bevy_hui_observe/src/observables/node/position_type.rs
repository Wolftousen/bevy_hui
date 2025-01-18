use bevy::prelude::*;
use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiPositionType;

impl HuiObservationEvent<HuiPositionType> {
    pub fn new(value: PositionType) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::PositionType(value.clone()),
        }
    }
}

pub fn observe_position_type(
    trigger: Trigger<HuiObservationEvent<HuiPositionType>>,
    mut query: Query<&mut Node>,
) {
    let Ok(mut comp) = query.get_mut(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::PositionType(s) => s,
        _ => return,
    };

    comp.position_type = value.clone();
}
