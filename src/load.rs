use bevy::asset::LoadState;
use bevy::prelude::*;
use crate::bloons_config::BloonsConfig;
use bevy_common_assets::ron::RonAssetPlugin;
use crate::AppState;
use crate::AppState::*;

pub(super) struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                RonAssetPlugin::<BloonsConfig>::new(&["config.ron"])
            )
            .add_systems(
                OnEnter(Load),
                start_load
            )
            .add_systems(
                Update,
                update_load.run_if(in_state(Load))
            )
        ;
    }
}

#[derive(Resource)]
struct LoadingConfig {
    handle: Handle<BloonsConfig>
}

fn start_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(LoadingConfig {
        handle: asset_server.load("bloons.config.ron")
    })
}

fn update_load(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<BloonsConfig>>,
    loading_config: Res<LoadingConfig>
) {
    let id = loading_config.handle.id();
    match asset_server.load_state(loading_config.handle.id()) {
        LoadState::Loaded => {
            let config = assets.remove(id).unwrap();
            commands.insert_resource(config);
            commands.remove_resource::<LoadingConfig>();
            next_state.set(Run);
        }
        LoadState::Failed => panic!("Failed to load config!"),
        _ => {}
    }
}