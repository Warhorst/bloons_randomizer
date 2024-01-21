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

#[derive(Deserialize)]
pub struct Hero {
    pub name: String,
    pub icon: String
}

#[derive(Deserialize)]
pub struct Tower {
    pub name: String,
    pub category: Category,
    pub icon: String
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub enum Category {
    Primary,
    Military,
    Magic,
    Support
}

#[derive(Deserialize)]
pub struct Map {
    pub name: String,
    pub icon: String
}

#[derive(Deserialize)]
pub struct Mode {
    pub name: String,
    pub icon: String
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::bloons_config::BloonsConfig;

    #[test]
    fn load_config_works() {
        let content = read_to_string("../assets/bloons.config.ron").unwrap();
        let _config = ron::from_str::<BloonsConfig>(content.as_str());
    }
}