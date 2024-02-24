use rand::rngs::ThreadRng;
use rand::thread_rng;
use crate::bloons_config::*;
use rand::seq::SliceRandom;
use crate::bloons_config::Category::*;
use crate::run::Settings;

pub fn random_select(
    config: &BloonsConfig,
    settings: &Settings
) -> Selection {
    let mut rng = thread_rng();

    let mode = config.modes.choose(&mut rng).expect("at least one mode should exist").clone();
    let map = config.maps.choose(&mut rng).expect("at least one map should exist").clone();
    let hero = config.heroes.choose(&mut rng).expect("at least one hero should exist").clone();
    let towers = [
        choose_towers(&mut rng, config, settings, Primary),
        choose_towers(&mut rng, config, settings, Military),
        choose_towers(&mut rng, config, settings, Magic),
        choose_towers(&mut rng, config, settings, Support),
    ].into_iter().flat_map(|ts| ts.into_iter()).collect();

    Selection {
        mode,
        map,
        hero,
        towers,
    }
}

fn choose_towers(
    rng: &mut ThreadRng,
    config: &BloonsConfig,
    settings: &Settings,
    category: Category
) -> Vec<Tower> {
    let towers = config.get_towers_of_category(category).into_iter().collect::<Vec<_>>();
    towers
        .choose_multiple(rng, settings.get_amount(category) as usize)
        .map(|t| (*t).clone())
        .collect()
}

pub struct Selection {
    pub mode: Mode,
    pub map: Map,
    pub hero: Hero,
    pub towers: Vec<Tower>
}