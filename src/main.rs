use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::load::LoadPlugin;
use crate::run::RunPlugin;

pub mod bloons_config;
mod load;
mod run;
mod random_select;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (800.0, 640.0).into(),
                        title: "bloons_randomizer".to_string(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
        )
        .add_state::<AppState>()
        .add_plugins((
            // WorldInspectorPlugin::new(),
            LoadPlugin,
            RunPlugin
        ))
        .run()
    ;
}

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Load,
    Run,
}