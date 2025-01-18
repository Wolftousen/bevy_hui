// use bevy::prelude::*;
// use super::super::{HuiObservationEvent, HuiObservationValue};

// pub struct HuiGridTemplateColumns;

// impl HuiObservationEvent<HuiGridTemplateColumns> {
//     pub fn new(value: Vec<RepeatedGridTrack>) -> Self {
//         HuiObservationEvent {
//             _marker: core::marker::PhantomData,
//             value: HuiObservationValue::Vec<RepeatedGridTrack>(value.clone()),
//         }
//     }
// }

// pub fn observe_grid_template_columns(
//     trigger: Trigger<HuiObservationEvent<HuiGridTemplateColumns>>,
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

//     comp.grid_template_columns = value.clone();
// }
