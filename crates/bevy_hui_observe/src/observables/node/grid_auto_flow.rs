// use bevy::prelude::*;
// use super::super::{HuiObservationEvent, HuiObservationValue};

// pub struct HuiGridAutoFlow;

// impl HuiObservationEvent<HuiGridAutoFlow> {
//     pub fn new(value: GridAutoFlow) -> Self {
//         HuiObservationEvent {
//             _marker: core::marker::PhantomData,
//             value: HuiObservationValue::GridAutoFlow(value.clone()),
//         }
//     }
// }

// pub fn observe_grid_auto_flow(
//     trigger: Trigger<HuiObservationEvent<HuiGridAutoFlow>>,
//     mut query: Query<&mut Node>,
// ) {
//     let Ok(mut comp) = query.get_mut(trigger.entity()) else {
//         return;
//     };

//     let observed = trigger.event();

//     let value = match &observed.value {
//         HuiObservationValue::GridAutoFlow(s) => s,
//         _ => return,
//     };

//     comp.grid_auto_flow = value.clone();
// }
