use crate::bloons_config::Category;
use crate::bloons_config::Category::*;

#[derive(Default)]
pub struct Settings {
    pub num_primary: usize,
    pub num_military: usize,
    pub num_magic: usize,
    pub num_support: usize,
}

impl Settings {
    pub fn get_amount(&self, category: Category) -> u8 {
        (match category {
            Primary => self.num_primary,
            Military => self.num_military,
            Magic => self.num_magic,
            Support => self.num_support
        }) as u8
    }
}