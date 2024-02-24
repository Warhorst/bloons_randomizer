use serde::Deserialize;

/// Every setting which will be taken in consideration when randomizing.
#[derive(Deserialize, Clone, Default)]
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

    pub fn num_towers_of_category(&self, category: Category) -> usize {
        self.get_towers_of_category(category).into_iter().count()
    }
}

#[derive(Deserialize, Clone, Default)]
pub struct Hero {
    pub name: String,
    pub icon: String
}

#[derive(Deserialize, Clone, Default)]
pub struct Tower {
    pub name: String,
    pub category: Category,
    pub icon: String
}

#[derive(Deserialize, Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Category {
    #[default]
    Primary,
    Military,
    Magic,
    Support
}

#[derive(Deserialize, Clone, Default)]
pub struct Map {
    pub name: String,
    pub icon: String
}

#[derive(Deserialize, Clone, Default)]
pub struct Mode {
    pub name: String,
    pub icon: String
}