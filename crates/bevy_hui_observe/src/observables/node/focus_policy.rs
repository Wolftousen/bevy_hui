use bevy::{prelude::*, ui::FocusPolicy};
use crate::prelude::HuiObservableType;

use super::super::{HuiObservationEvent, HuiObservationValue};

pub struct HuiFocusPolicy;

impl HuiObservationEvent<HuiFocusPolicy> {
    pub fn new(value: FocusPolicy) -> Self {
        HuiObservationEvent {
            _marker: core::marker::PhantomData,
            value: HuiObservationValue::FocusPolicy(value.clone()),
        }
    }
}

impl HuiObservableType for HuiFocusPolicy {
    fn observe(
        &self,
        entity_commands: &mut EntityCommands,
    ) {
        entity_commands.observe(observe_focus_policy);
    }
}

pub fn observe_focus_policy(
    trigger: Trigger<HuiObservationEvent<HuiFocusPolicy>>,
    mut commands: Commands,
    query: Query<Entity, With<FocusPolicy>>,
) {
    let Ok(entity) = query.get(trigger.entity()) else {
        return;
    };

    let observed = trigger.event();

    let value = match &observed.value {
        HuiObservationValue::FocusPolicy(s) => s,
        _ => return,
    };

    commands.entity(entity).insert(value.clone());
}
