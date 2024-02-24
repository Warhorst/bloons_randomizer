use crate::bloons_config::{Hero, Map, Mode, Tower};

#[derive(Default)]
pub struct Selection {
    pub mode: Mode,
    pub map: Map,
    pub hero: Hero,
    pub towers: Vec<Tower>
}