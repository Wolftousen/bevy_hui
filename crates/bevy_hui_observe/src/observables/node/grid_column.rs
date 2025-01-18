// use bevy::prelude::*;
// use super::super::{HuiObservationEvent, HuiObservationValue};

// pub struct HuiGridColumn;

// impl HuiObservationEvent<HuiGridColumn> {
//     pub fn new(value: GridPlacement) -> Self {
//         HuiObservationEvent {
//             _marker: core::marker::PhantomData,
//             value: HuiObservationValue::GridPlacement(value.clone()),
//         }
//     }
// }

// pub fn observe_grid_column(
//     trigger: Trigger<HuiObservationEvent<HuiGridColumn>>,
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

//     comp.grid_column = value.clone();
// }
