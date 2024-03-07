use std::collections::HashSet;
use crate::bloons_config::{BloonsConfig, Category, Hero, Map, Mode, Tower};
use crate::bloons_config::Category::*;

#[derive(Default)]
pub struct Settings {
    pub num_primary: usize,
    pub num_military: usize,
    pub num_magic: usize,
    pub num_support: usize,
    pub active_maps: HashSet<Map>,
    pub active_modes: HashSet<Mode>,
    pub active_heroes: HashSet<Hero>,
    pub active_towers: HashSet<Tower>
}

impl Settings {
    pub fn new(config: &BloonsConfig) -> Self {
        Settings {
            num_primary: 0,
            num_military: 0,
            num_magic: 0,
            num_support: 0,
            active_maps: config.maps.iter().cloned().collect(),
            active_modes: config.modes.iter().cloned().collect(),
            active_heroes: config.heroes.iter().cloned().collect(),
            active_towers: config.towers.iter().cloned().collect()
        }
    }

    /// Get the current amount of selected towers
    pub fn get_amount(&self, category: Category) -> usize {
        match category {
            Primary => self.num_primary,
            Military => self.num_military,
            Magic => self.num_magic,
            Support => self.num_support
        }
    }

    /// Get the maximum amount of possible towers for the given category
    pub fn get_max(&self, category: Category) -> usize {
        self.active_towers.iter().filter(|t| t.category == category).count()
    }
}