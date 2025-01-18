// use bevy::prelude::*;
// use super::super::{HuiObservationEvent, HuiObservationValue};

// pub struct HuiGridTemplateRows;

// impl HuiObservationEvent<HuiGridTemplateRows> {
//     pub fn new(value: Vec<RepeatedGridTrack>) -> Self {
//         HuiObservationEvent {
//             _marker: core::marker::PhantomData,
//             value: HuiObservationValue::Vec<RepeatedGridTrack>(value.clone()),
//         }
//     }
// }

// pub fn observe_grid_template_rows(
//     trigger: Trigger<HuiObservationEvent<HuiGridTemplateRows>>,
//     mut query: Query<&mut Node>,
// ) {
//     let Ok(mut comp) = query.get_mut(trigger.entity()) else {
//         return;
//     };

//     let observed = trigger.event();

//     let value = match &observed.value {
//         HuiObservationValue::Vec<RepeatedGridTrack>(s) => s,
//         _ => return,
//     };

//     comp.grid_template_rows = value.clone();
// }
