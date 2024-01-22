use bevy::prelude::*;
use serde::Deserialize;

/// Every setting which will be taken in consideration when randomizing.
#[derive(Resource, Asset, TypePath, Deserialize)]
pub struct BloonsConfig {
    pub heroes: Vec<Hero>,
    pub towers: Vec<Tower>,
    pub maps: Vec<Map>,
    pub modes: Vec<Mode>
}

impl BloonsConfig {
    pub fn get_towers_of_category(&self, category: Category) -> impl IntoIterator<Item=&Tower> {
        self.towers.iter().filter(move |t| t.category == category)
    }
}

#[derive(Deserialize, Clone)]
pub struct Hero {
    pub name: String,
    pub icon: String
}

#[derive(Deserialize, Clone)]
pub struct Tower {
    pub name: String,
    pub category: Category,
    pub icon: String
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Category {
    Primary,
    Military,
    Magic,
    Support
}

#[derive(Deserialize, Clone)]
pub struct Map {
    pub name: String,
    pub icon: String
}

#[derive(Deserialize, Clone)]
pub struct Mode {
    pub name: String,
    pub icon: String
}