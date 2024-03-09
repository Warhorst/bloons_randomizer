use std::collections::HashSet;
use crate::bloons_config::{BloonsConfig, Category, Hero, Map, Mode, Tower};
use crate::bloons_config::Category::*;

/// The settings used to create a random selection
#[derive(Default)]
pub struct Settings {
    /// The amount of primary towers wished
    pub num_primary: usize,
    /// The amount of military towers wished
    pub num_military: usize,
    /// The amount of magic towers wished
    pub num_magic: usize,
    /// The amount of support towers wished
    pub num_support: usize,
    /// The map pool to chose from
    pub active_maps: HashSet<Map>,
    /// The mode pool to chose from
    pub active_modes: HashSet<Mode>,
    /// The hero pool to chose from
    pub active_heroes: HashSet<Hero>,
    /// The tower pool to chose from
    pub active_towers: HashSet<Tower>,
    /// Tells how the allowed paths for towers should be chosen
    pub path_restriction_setting: PathRestrictionSetting
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
            active_towers: config.towers.iter().cloned().collect(),
            path_restriction_setting: PathRestrictionSetting::default()
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

/// Defines how allowed paths for towers should be set
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum PathRestrictionSetting {
    /// No path setting, all paths are allowed
    #[default]
    None,
    /// Create one restriction for all towers
    Global,
    /// Restrict different paths for each tower
    Custom
}