use crate::bloons_config::{Hero, Map, Mode, Tower};

/// The random selection of settings for the game
#[derive(Default)]
pub struct Selection {
    /// The chosen mode, if any
    pub mode: Option<Mode>,
    /// The chosen map, if any
    pub map: Option<Map>,
    /// The chosen hero, if any
    pub hero: Option<Hero>,
    /// All chosen towers with path restrictions
    pub towers: Vec<(Tower, Option<PathRestriction>)>
}

/// A restriction for path upgrades a tower can have
#[derive(Copy, Clone)]
pub struct PathRestriction {
    /// max level of the top path
    pub top_max: usize,
    /// max level of the center path
    pub center_max: usize,
    /// mal level of the bottom path
    pub bottom_max: usize
}