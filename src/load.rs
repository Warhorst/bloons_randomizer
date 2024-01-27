use bevy::prelude::*;
use bevy_asset_preload::AssetPreloadUpdate;
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
                OnEnter(Load),
                spawn_load_progress,
            )
            .add_systems(
                Update,
                update_load_progress.run_if(in_state(Load)),
            )
            .add_systems(
                OnExit(Load),
                (
                    despawn_load_progress,
                    insert_config,
                ),
            )
        ;
    }
}

/// Identifies the ui that tells the load progress of all assets.
#[derive(Component)]
struct LoadProgress;

fn spawn_load_progress(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        TextBundle::from_section(
            "Loading assets: ",
            TextStyle {
                font: asset_server.load("font/FiraSans-Bold.ttf"),
                font_size: 25.0,
                ..default()
            },
        )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            }),
        LoadProgress,
    ));
}

fn update_load_progress(
    mut event_reader: EventReader<AssetPreloadUpdate>,
    mut query: Query<&mut Text, With<LoadProgress>>,
) {
    for update in event_reader.read() {
        let num_loaded = update.num_loaded;
        let num_loading = update.num_loading;

        for mut text in &mut query {
            text.sections[0].value = format!("Loading assets: {num_loaded}/{num_loading}")
        }
    }
}

fn despawn_load_progress(
    mut commands: Commands,
    query: Query<Entity, With<LoadProgress>>,
) {
    for e in &query {
        commands.entity(e).despawn();
    }
}

fn insert_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    configs: Res<Assets<BloonsConfig>>,
) {
    let handle = asset_server.load("bloons.config.ron");
    let config = configs.get(handle).expect("the config should be loaded at this point").clone();
    commands.insert_resource(config);
}