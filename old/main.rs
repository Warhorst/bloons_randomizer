use bevy::prelude::*;
use bevy_asset_preload::{AssetPreloadPlugin, load_assets};
use crate::AppState::{Load, Run};
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
                        resolution: (1400.0, 900.0).into(),
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
            AssetPreloadPlugin::load_given_paths(Load, Run, load_assets!()),
            LoadPlugin,
            RunPlugin
        ))
        .add_systems(
            Startup,
            spawn_camera
        )
        .run()
    ;
}

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Load,
    Run,
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}