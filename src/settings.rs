use std::collections::HashSet;
use crate::bloons_config::{BloonsConfig, Category, Hero};
use crate::bloons_config::Category::*;

#[derive(Default)]
pub struct Settings {
    pub num_primary: usize,
    pub num_military: usize,
    pub num_magic: usize,
    pub num_support: usize,
    pub active_heroes: HashSet<Hero>
}

impl Settings {
    pub fn new(config: &BloonsConfig) -> Self {
        Settings {
            num_primary: 0,
            num_military: 0,
            num_magic: 0,
            num_support: 0,
            active_heroes: config.heroes.iter().cloned().collect(),
        }
    }

    pub fn get_amount(&self, category: Category) -> u8 {
        (match category {
            Primary => self.num_primary,
            Military => self.num_military,
            Magic => self.num_magic,
            Support => self.num_support
        }) as u8
    }
}