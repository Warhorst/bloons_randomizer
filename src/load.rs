use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::AppState::*;
use crate::bloons_config::BloonsConfig;

pub(super) struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                RonAssetPlugin::<BloonsConfig>::new(&["config.ron"])
            )
            .add_systems(
                OnExit(Load),
                insert_config
            )
        ;
    }
}

fn insert_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    configs: Res<Assets<BloonsConfig>>
) {
    let handle = asset_server.load("bloons.config.ron");
    let config = configs.get(handle).expect("the config should be loaded at this point").clone();
    commands.insert_resource(config);
}