use bevy::prelude::*;
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
                        resolution: (1600.0, 1000.0).into(),
                        title: "Bloons Randomizer".to_string(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
        )
        .add_state::<AppState>()
        .add_plugins((
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