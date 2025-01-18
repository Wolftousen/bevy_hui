// use bevy::prelude::*;
// use super::super::{HuiObservationEvent, HuiObservationValue};

// pub struct HuiGridRow;

// impl HuiObservationEvent<HuiGridRow> {
//     pub fn new(value: GridPlacement) -> Self {
//         HuiObservationEvent {
//             _marker: core::marker::PhantomData,
//             value: HuiObservationValue::GridPlacement(value.clone()),
//         }
//     }
// }

// pub fn observe_grid_row(
//     trigger: Trigger<HuiObservationEvent<HuiGridRow>>,
//     mut query: Query<&mut Node>,
// ) {
//     let Ok(mut comp) = query.get_mut(trigger.entity()) else {
//         return;
//     };

//     let observed = trigger.event();

//     let value = match &observed.value {
//         HuiObservationValue::GridPlacement(s) => s,
//         _ => return,
//     };

//     comp.grid_row = value.clone();
// }
