use bevy::prelude::*;

mod observables;
mod watcher;

pub mod prelude {
    pub use crate::observables::*;
    pub use crate::watcher::*;
}

pub struct HuiObservePlugin;

impl Plugin for HuiObservePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::watcher::HuiPropertyWatchers>();
    }
}