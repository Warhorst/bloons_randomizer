use serde::Deserialize;

/// Every setting which will be taken in consideration when randomizing.
#[derive(Deserialize, Clone)]
pub struct BloonsConfig {
    pub heroes: Vec<Hero>,
    pub towers: Vec<Tower>,
    pub maps: Vec<Map>,
    pub modes: Vec<Mode>
}

impl Default for BloonsConfig {
    /// Include the config file in the binary and parse it
    fn default() -> Self {
        let bytes = include_bytes!("../assets/bloons.config.ron").to_vec();
        let config_string = String::from_utf8(bytes).expect("config should be valid UTF8");
        ron::from_str::<BloonsConfig>(&config_string).expect("the config should be valid RON")
    }
}

impl BloonsConfig {
    pub fn get_towers_of_category(&self, category: Category) -> impl IntoIterator<Item=&Tower> {
        self.towers.iter().filter(move |t| t.category == category)
    }

    pub fn get_modes_of_difficulty(&self, difficulty: ModeDifficulty) -> impl IntoIterator<Item=&Mode> {
        self.modes.iter().filter(move |m| m.difficulty == difficulty)
    }
}

#[derive(Deserialize, Clone, Default, Eq, PartialEq, Hash)]
pub struct Hero {
    pub name: String,
    pub icon: String
}

#[derive(Deserialize, Clone, Default, Eq, PartialEq, Hash)]
pub struct Tower {
    pub name: String,
    pub category: Category,
    pub icon: String
}

#[derive(Deserialize, Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub enum Category {
    #[default]
    Primary,
    Military,
    Magic,
    Support
}

#[derive(Deserialize, Clone, Default, Eq, PartialEq, Hash)]
pub struct Map {
    pub name: String,
    pub icon: String,
    pub difficulty: MapDifficulty
}

#[derive(Deserialize, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub enum MapDifficulty {
    #[default]
    Beginner,
    Intermediate,
    Advanced,
    Expert
}

#[derive(Deserialize, Clone, Default, Eq, PartialEq, Hash)]
pub struct Mode {
    pub name: String,
    pub icon: String,
    pub difficulty: ModeDifficulty
}

#[derive(Deserialize, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub enum ModeDifficulty {
    #[default]
    Easy,
    Medium,
    Hard
}