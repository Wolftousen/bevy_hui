// use bevy::prelude::*;
// use super::super::{HuiObservationEvent, HuiObservationValue};

// pub struct HuiGridAutoColumns;

// impl HuiObservationEvent<HuiGridAutoColumns> {
//     pub fn new(value: Vec<GridTrack>) -> Self {
//         HuiObservationEvent {
//             _marker: core::marker::PhantomData,
//             value: HuiObservationValue::Vec<GridTrack>(value.clone()),
//         }
//     }
// }

// pub fn observe_grid_auto_columns(
//     trigger: Trigger<HuiObservationEvent<HuiGridAutoColumns>>,
//     mut query: Query<&mut Node>,
// ) {
//     let Ok(mut comp) = query.get_mut(trigger.entity()) else {
//         return;
//     };

//     let observed = trigger.event();

//     let value = match &observed.value {
//         HuiObservationValue::Vec<GridTrack>(s) => s,
//         _ => return,
//     };

//     comp.grid_auto_columns = value.clone();
// }
