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

impl BloonsConfig {
    pub fn get_towers_of_category(&self, category: Category) -> impl IntoIterator<Item=&Tower> {
        self.towers.iter().filter(move |t| t.category == category)
    }
}

#[derive(Deserialize, Clone)]
pub struct Hero {
    pub name: String,
    pub icon: String
}

#[derive(Deserialize, Clone)]
pub struct Tower {
    pub name: String,
    pub category: Category,
    pub icon: String
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Category {
    Primary,
    Military,
    Magic,
    Support
}

#[derive(Deserialize, Clone)]
pub struct Map {
    pub name: String,
    pub icon: String
}

#[derive(Deserialize, Clone)]
pub struct Mode {
    pub name: String,
    pub icon: String
}

#[cfg(test)]
mod tests {
    use std::fs::{read_dir, read_to_string};

    use crate::bloons_config::BloonsConfig;

    #[test]
    fn load_config_works() {
        let content = read_to_string("../assets/bloons.config.ron").unwrap();
        let _config = ron::from_str::<BloonsConfig>(content.as_str());
    }

    #[test]
    fn foo() {
        let dir = read_dir("./assets/heroes/").unwrap();

        for file in dir {
            let file = file.unwrap();
            let file_name = file.file_name().to_str().unwrap().to_string();
            let name = make_first_letters_big(&file_name.replace(".webp", "").replace("_", " "));
            println!("Hero(");
            println!("    name: \"{name}\",");
            println!("    icon: \"heroes/{file_name}\"");
            println!("),");
        }
    }

    fn make_first_letters_big(string: &String) -> String {
        let mut prev_was_space = false;

        string
            .chars()
            .enumerate()
            .map(|(i, char)| {
                if i == 0 {
                    char.to_ascii_uppercase()
                } else if char == ' ' {
                    prev_was_space = true;
                    char
                } else if prev_was_space {
                    prev_was_space = false;
                    char.to_ascii_uppercase()
                } else {
                    char
                }
            })
            .collect()
    }
}