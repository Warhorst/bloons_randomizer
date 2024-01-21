use bevy::prelude::*;
use bevy::prelude::PositionType::Absolute;
use crate::AppState::Run;
use crate::bloons_config::Category;
use crate::bloons_config::Category::*;

pub(super) struct RunPlugin;

impl Plugin for RunPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(Run),
                (
                    spawn_camera,
                    spawn_ui
                ),
            )
        ;
    }
}

#[derive(Component)]
struct TowerAmountText(pub Category);

#[derive(Component)]
struct TowerAmountUp(pub Category);

#[derive(Component)]
struct TowerAmountDown(pub Category);

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(700.0),
                height: Val::Px(200.0),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            spawn_tower_slider(parent, &asset_server, 0.0, Primary);
            spawn_tower_slider(parent, &asset_server, 20.0, Military);
            spawn_tower_slider(parent, &asset_server, 40.0, Magic);
            spawn_tower_slider(parent, &asset_server, 60.0, Support);
            spawn_randomize_button(parent, &asset_server, 80.0);
        })
    ;
}

fn spawn_tower_slider(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    top_offset: f32,
    category: Category,
) {
    let text = format!("{:?}", category);

    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(40.0),
                height: Val::Percent(20.0),
                top: Val::Percent(top_offset),
                position_type: Absolute,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                NodeBundle {
                    style: Style {
                        width: Val::Percent(40.0),
                        height: Val::Px(100.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        text,
                        TextStyle {
                            font: asset_server.load("font/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            parent.spawn(
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(20.0),
                        height: Val::Percent(100.0),
                        left: Val::Percent(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: Absolute,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                }
            ).with_children(|parent| {
                parent.spawn((
                    TowerAmountDown(category),
                    TextBundle::from_section(
                        "<",
                        TextStyle {
                            font: asset_server.load("font/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )
                ));
            });

            parent.spawn(
                NodeBundle {
                    style: Style {
                        width: Val::Percent(20.0),
                        height: Val::Percent(100.0),
                        left: Val::Percent(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: Absolute,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TowerAmountText(category),
                        TextBundle::from_section(
                            "0",
                            TextStyle {
                                font: asset_server.load("font/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        )
                    ));
                });

            parent.spawn(
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(20.0),
                        height: Val::Percent(100.0),
                        left: Val::Percent(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: Absolute,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                }
            ).with_children(|parent| {
                parent.spawn((
                    TowerAmountUp(category),
                    TextBundle::from_section(
                        ">",
                        TextStyle {
                            font: asset_server.load("font/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )));
            });
        })
    ;
}

fn spawn_randomize_button(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    top_offset: f32
) {
    parent
        .spawn((
            Name::new("ReturnButton"),
            ButtonBundle {
                style: Style {
                    width: Val::Percent(40.0),
                    height: Val::Percent(20.0),
                    top: Val::Percent(top_offset),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: Absolute,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            }))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Randomize",
                TextStyle {
                    font: asset_server.load("font/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}