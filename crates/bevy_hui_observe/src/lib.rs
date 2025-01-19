use bevy::prelude::*;

mod observables;
mod watcher;

use observables::HuiObservableProperties;
use watcher::HuiPropertyWatchers;

pub mod prelude {
    pub use crate::observables::*;
    pub use crate::watcher::*;
}

pub struct HuiObservePlugin;

impl Plugin for HuiObservePlugin {
    fn build(&self, app: &mut App) {
        let observables = HuiObservableProperties::initialize();

        app
            .init_resource::<HuiPropertyWatchers>()
            .insert_resource(observables);
    }
}