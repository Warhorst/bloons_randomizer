use eframe::{App, Frame, Theme};
use eframe::epaint::FontFamily;
use egui::{CentralPanel, Color32, Context, FontId, Grid, Image, ImageButton, ScrollArea, TextStyle, Ui, Vec2};
use rand::prelude::IteratorRandom;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::bloons_config::{BloonsConfig, Category, Tower};
use crate::bloons_config::Category::*;
use crate::bloons_config::Difficulty::*;
use crate::images::Images;
use crate::selection::Selection;
use crate::settings::Settings;

mod bloons_config;
mod settings;
mod selection;
mod images;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_fullscreen(true),
        follow_system_theme: false,
        default_theme: Theme::Dark,
        ..Default::default()
    };

    let bloons_config = BloonsConfig::default();
    let images = Images::default();
    let settings = Settings::new(&bloons_config);

    eframe::run_native(
        "Bloons Randomizer",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<BloonsRandomizerApp<'_>>::new(BloonsRandomizerApp {
                bloons_config,
                images,
                settings,
                selection: None,
            })
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    let options = eframe::WebOptions {
        follow_system_theme: false,
        default_theme: Theme::Dark,
        ..Default::default()
    };

    let bloons_config = BloonsConfig::default();
    let images = Images::default();
    let settings = Settings::new(&bloons_config);

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                options,
                Box::new(|cc| {
                    egui_extras::install_image_loaders(&cc.egui_ctx);
                    Box::<BloonsRandomizerApp<'_>>::new(BloonsRandomizerApp {
                        bloons_config,
                        images,
                        settings,
                        selection: None,
                    })
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}

#[derive(Default)]
struct BloonsRandomizerApp<'a> {
    bloons_config: BloonsConfig,
    images: Images<'a>,
    settings: Settings,
    selection: Option<Selection>,
}

impl<'a> BloonsRandomizerApp<'a> {
    const SELECTED_COLOR: Color32 = Color32::WHITE;
    const UNSELECTED_COLOR: Color32 = Color32::DARK_GRAY;

    pub fn random_select(&mut self) {
        let mut rng = thread_rng();

        let mode = self.settings.active_modes.iter().choose(&mut rng).cloned();
        let map = self.settings.active_maps.iter().choose(&mut rng).cloned();
        let hero = self.settings.active_heroes.iter().choose(&mut rng).cloned();
        let towers = [
            self.choose_towers(&mut rng, Primary),
            self.choose_towers(&mut rng, Military),
            self.choose_towers(&mut rng, Magic),
            self.choose_towers(&mut rng, Support),
        ].into_iter().flat_map(|ts| ts.into_iter()).collect();

        self.selection = Some(Selection {
            mode,
            map,
            hero,
            towers,
        })
    }

    fn choose_towers(
        &self,
        rng: &mut ThreadRng,
        category: Category,
    ) -> Vec<Tower> {
        let towers = self.settings.active_towers.iter().filter(|t| t.category == category).collect::<Vec<_>>();
        towers
            .choose_multiple(rng, self.settings.get_amount(category))
            .map(|t| (*t).clone())
            .collect()
    }
}

impl<'a> App for BloonsRandomizerApp<'a> {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.style_mut().text_styles.insert(
                    TextStyle::Heading,
                    FontId::new(36.0, FontFamily::Proportional),
                );
                ui.style_mut().text_styles.insert(
                    TextStyle::Body,
                    FontId::new(24.0, FontFamily::Proportional),
                );
                ui.style_mut().text_styles.insert(
                    TextStyle::Button,
                    FontId::new(24.0, FontFamily::Proportional),
                );

                ui.heading("Bloons Randomizer");

                self.create_monkey_amount_sliders(ui);
                self.create_include_exclude_modes_ui(ui);
                self.create_include_exclude_maps_ui(ui);
                self.create_include_exclude_heroes_ui(ui);
                self.create_include_exclude_towers_ui(ui);

                if ui.button("Randomize").clicked() {
                    self.random_select();
                }

                let selection = match &self.selection {
                    Some(s) => s,
                    None => return,
                };

                if let Some(mode) = &selection.mode {
                    ui.horizontal(|ui| {
                        ui.add_sized([75.0, 75.0], Image::new(self.images.get_image(&mode.icon)));
                        ui.label(&mode.name);
                    });
                }

                if let Some(map) = &selection.map {
                    ui.add(Image::new(self.images.get_image(&map.icon)).max_size(Vec2::new(450.0, 300.0)));
                }

                if let Some(hero) = &selection.hero {
                    ui.add(Image::new(self.images.get_image(&hero.icon)).max_size(Vec2::new(300.0, 150.0)));
                }

                Grid::new("monkey selection").show(ui, |ui| {
                    selection.towers
                        .iter()
                        .enumerate()
                        .for_each(|(i, tower)| {
                            ui.add_sized([75.0, 75.0], Image::new(self.images.get_image(&tower.icon)));
                            if (i + 1) % 5 == 0 {
                                ui.end_row();
                            }
                        })
                });
            });
        });
    }
}

impl<'a> BloonsRandomizerApp<'a> {
    fn create_monkey_amount_sliders(&mut self, ui: &mut Ui) {
        Grid::new("monkey amount sliders").show(ui, |ui| {
            ui.style_mut().text_styles.insert(
                TextStyle::Body,
                FontId::new(24.0, FontFamily::Proportional),
            );

            ui.label("Primary: ");
            let max_primary = self.settings.get_max(Primary);
            ui.add(egui::Slider::new(&mut self.settings.num_primary, 0..=max_primary));
            ui.end_row();
            ui.label("Military: ");
            let max_military = self.settings.get_max(Military);
            ui.add(egui::Slider::new(&mut self.settings.num_military, 0..=max_military));
            ui.end_row();
            ui.label("Magic: ");
            let max_magic = self.settings.get_max(Magic);
            ui.add(egui::Slider::new(&mut self.settings.num_magic, 0..=max_magic));
            ui.end_row();
            ui.label("Support: ");
            let max_support = self.settings.get_max(Support);
            ui.add(egui::Slider::new(&mut self.settings.num_support, 0..=max_support));
        });
    }

    fn create_include_exclude_modes_ui(&mut self, ui: &mut Ui) {
        ui.collapsing("Include/Exclude Modes", |ui| {
            Grid::new("mode include exclude").show(ui, |ui| {
                [Easy, Medium, Hard].into_iter()
                    .for_each(|d| {
                        self.bloons_config.get_modes_of_difficulty(d)
                            .into_iter()
                            .for_each(|mode| {
                                let currently_selected = self.settings.active_modes.contains(mode);
                                let tint = match currently_selected {
                                    true => Self::SELECTED_COLOR,
                                    false => Self::UNSELECTED_COLOR
                                };

                                if ui.add_sized(
                                    [75.0, 75.0],
                                    ImageButton::new(self.images.get_image(&mode.icon)).tint(tint),
                                ).clicked() {
                                    match currently_selected {
                                        true => { self.settings.active_modes.remove(mode); }
                                        false => { self.settings.active_modes.insert(mode.clone()); }
                                    }
                                }
                            });

                        ui.end_row();
                    });
            });
        });
    }

    fn create_include_exclude_maps_ui(&mut self, ui: &mut Ui) {
        ui.collapsing("Include/Exclude Maps", |ui| {
            Grid::new("map include exclude").show(ui, |ui| {
                self.bloons_config.maps
                    .iter()
                    .enumerate()
                    .for_each(|(i, map)| {
                        let currently_selected = self.settings.active_maps.contains(map);
                        let tint = match currently_selected {
                            true => Self::SELECTED_COLOR,
                            false => Self::UNSELECTED_COLOR
                        };

                        if ui.add_sized(
                            [125.0, 100.0],
                            ImageButton::new(self.images.get_image(&map.icon)).tint(tint),
                        ).clicked() {
                            match currently_selected {
                                true => { self.settings.active_maps.remove(map); }
                                false => { self.settings.active_maps.insert(map.clone()); }
                            }
                        }

                        if (i + 1) % 5 == 0 {
                            ui.end_row();
                        }
                    })
            });
        });
    }

    fn create_include_exclude_heroes_ui(&mut self, ui: &mut Ui) {
        ui.collapsing("Include/Exclude Heroes", |ui| {
            Grid::new("hero include exclude").show(ui, |ui| {
                self.bloons_config.heroes
                    .iter()
                    .enumerate()
                    .for_each(|(i, hero)| {
                        let currently_selected = self.settings.active_heroes.contains(hero);
                        let tint = match currently_selected {
                            true => Self::SELECTED_COLOR,
                            false => Self::UNSELECTED_COLOR
                        };

                        if ui.add_sized(
                            [75.0, 75.0],
                            ImageButton::new(self.images.get_image(&hero.icon)).tint(tint),
                        ).clicked() {
                            match currently_selected {
                                true => { self.settings.active_heroes.remove(hero); }
                                false => { self.settings.active_heroes.insert(hero.clone()); }
                            }
                        }

                        if (i + 1) % 5 == 0 {
                            ui.end_row();
                        }
                    })
            });
        });
    }

    fn create_include_exclude_towers_ui(&mut self, ui: &mut Ui) {
        ui.collapsing("Include/Exclude Towers", |ui| {
            Grid::new("tower include exclude").show(ui, |ui| {
                [Primary, Military, Magic, Support].into_iter()
                    .for_each(|c| {
                        self.bloons_config.get_towers_of_category(c)
                            .into_iter()
                            .for_each(|tower| {
                                let currently_selected = self.settings.active_towers.contains(tower);
                                let tint = match currently_selected {
                                    true => Self::SELECTED_COLOR,
                                    false => Self::UNSELECTED_COLOR
                                };

                                if ui.add_sized(
                                    [75.0, 75.0],
                                    ImageButton::new(self.images.get_image(&tower.icon)).tint(tint),
                                ).clicked() {
                                    match currently_selected {
                                        true => { self.settings.active_towers.remove(tower); }
                                        false => { self.settings.active_towers.insert(tower.clone()); }
                                    }
                                }
                            });

                        ui.end_row();
                    })
            });
        });
    }
}