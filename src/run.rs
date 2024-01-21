use bevy::prelude::*;
use bevy::prelude::PositionType::Absolute;
use crate::AppState::Run;
use crate::bloons_config::{BloonsConfig, Category, Hero, Map, Mode, Tower};
use crate::bloons_config::Category::*;
use crate::random_select::{random_select, Selection};

pub(super) struct RunPlugin;

impl Plugin for RunPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Settings>()
            .add_systems(
                OnEnter(Run),
                (
                    spawn_camera,
                    spawn_settings_ui
                ),
            )
            .add_systems(
                Update,
                (
                    process_randomize_pressed,
                    process_increase_tower_pressed,
                    process_decrease_tower_pressed
                ).run_if(in_state(Run)),
            )
        ;
    }
}

#[derive(Resource, Default)]
pub struct Settings {
    num_primary: u8,
    num_military: u8,
    num_magic: u8,
    num_support: u8,
}

impl Settings {
    pub fn get_amount(&self, category: Category) -> u8 {
        match category {
            Primary => self.num_primary,
            Military => self.num_military,
            Magic => self.num_magic,
            Support => self.num_support
        }
    }

    fn set_amount(&mut self, category: Category, amount: u8) {
        match category {
            Primary => self.num_primary = amount,
            Military => self.num_military = amount,
            Magic => self.num_magic = amount,
            Support => self.num_support = amount,
        }
    }
}

#[derive(Component, Deref)]
struct TowerAmountText(pub Category);

#[derive(Component, Deref)]
struct TowerAmountUp(pub Category);

#[derive(Component, Deref)]
struct TowerAmountDown(pub Category);

#[derive(Component)]
struct RandomizeButton;

#[derive(Component)]
struct SelectionUi;

fn process_randomize_pressed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<BloonsConfig>,
    settings: Res<Settings>,
    selection_uis: Query<Entity, With<SelectionUi>>,
    button_query: Query<&Interaction, (With<RandomizeButton>, Changed<Interaction>)>,
) {
    for interaction in &button_query {
        if let Interaction::Pressed = *interaction {
            for entity in &selection_uis {
                commands.entity(entity).despawn_recursive();
            }

            let selection = random_select(&config, &settings);
            spawn_selection_ui(
                &mut commands,
                &asset_server,
                selection
            );
        }
    }
}

fn process_increase_tower_pressed(
    bloons_config: Res<BloonsConfig>,
    mut settings: ResMut<Settings>,
    button_query: Query<(&Interaction, &TowerAmountUp), Changed<Interaction>>,
    mut amount_text_query: Query<(&mut Text, &TowerAmountText)>,
) {
    for (interaction, amount_up) in &button_query {
        if let Interaction::Pressed = *interaction {
            let category = **amount_up;
            let max = bloons_config.get_towers_of_category(category).into_iter().count() as u8;
            let mut text = amount_text_query
                .iter_mut()
                .filter(|(_, amount_text)| ***amount_text == category)
                .map(|(text, _)| text)
                .next()
                .unwrap();
            let current = text.sections[0].value.parse::<u8>().unwrap();
            let new_current = u8::min(current + 1, max);
            settings.set_amount(category, new_current);
            text.sections[0].value = new_current.to_string();
        }
    }
}

fn process_decrease_tower_pressed(
    mut settings: ResMut<Settings>,
    button_query: Query<(&Interaction, &TowerAmountDown), Changed<Interaction>>,
    mut amount_text_query: Query<(&mut Text, &TowerAmountText)>,
) {
    for (interaction, amount_down) in &button_query {
        if let Interaction::Pressed = *interaction {
            let category = **amount_down;
            let mut text = amount_text_query
                .iter_mut()
                .filter(|(_, amount_text)| ***amount_text == category)
                .map(|(text, _)| text)
                .next()
                .unwrap();
            let current = text.sections[0].value.parse::<u8>().unwrap();
            let new_current = current.saturating_sub(1);
            settings.set_amount(category, new_current);
            text.sections[0].value = new_current.to_string();
        }
    }
}

fn spawn_selection_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    selection: Selection,
) {
    commands.spawn((
        SelectionUi,
        NodeBundle {
            style: Style {
                width: Val::Px(800.0),
                height: Val::Px(800.0),
                top: Val::Px(200.0),
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        }
    )).with_children(|parent| {
        spawn_mode_ui(parent, asset_server, &selection.mode);
        spawn_map_ui(parent, asset_server, &selection.map);
        spawn_hero_ui(parent, asset_server, &selection.hero);
        spawn_tower_ui(parent, asset_server, &selection.towers);
    });
}

fn spawn_mode_ui(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    mode: &Mode,
) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(20.0),
            justify_content: JustifyContent::Start,
            position_type: Absolute,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    position_type: Absolute,
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            UiImage::new(asset_server.load(mode.icon.clone())),
        ));

        parent.spawn(
            NodeBundle {
                style: Style {
                    width: Val::Percent(60.0),
                    height: Val::Px(100.0),
                    left: Val::Percent(40.0),
                    position_type: Absolute,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    mode.name.clone(),
                    TextStyle {
                        font: asset_server.load("font/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
    });
}

fn spawn_map_ui(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    map: &Map
) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                top: Val::Percent(20.0),
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        },
        UiImage::new(asset_server.load(map.icon.clone())),
    ));
}

fn spawn_hero_ui(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    hero: &Hero
) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                top: Val::Percent(60.0),
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        },
        UiImage::new(asset_server.load(hero.icon.clone())),
    ));
}

fn spawn_tower_ui(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    towers: &[Tower]
) {
    for (i, tower) in towers.iter().enumerate() {
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    top: Val::Percent(80.0),
                    left: Val::Px(50.0 * i as f32 ),
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            UiImage::new(asset_server.load(tower.icon.clone())),
        ));
    }
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_settings_ui(
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

            parent.spawn((
                TowerAmountDown(category),
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
                })
            ).with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "<",
                        TextStyle {
                            font: asset_server.load("font/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    )
                );
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

            parent.spawn((
                TowerAmountUp(category),
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
                })
            ).with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        ">",
                        TextStyle {
                            font: asset_server.load("font/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
            });
        })
    ;
}

fn spawn_randomize_button(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    top_offset: f32,
) {
    parent
        .spawn((
            Name::new("RandomizeButton"),
            RandomizeButton,
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