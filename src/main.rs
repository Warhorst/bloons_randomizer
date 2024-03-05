use eframe::{App, Frame, Theme};
use eframe::epaint::FontFamily;
use egui::{CentralPanel, Color32, Context, FontId, Grid, Image, ImageButton, ScrollArea, TextStyle, Vec2};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::bloons_config::{BloonsConfig, Category, Tower};
use crate::bloons_config::Category::*;
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
    const HERO_SELECTED_COLOR: Color32 = Color32::WHITE;
    const HERO_UNSELECTED_COLOR: Color32 = Color32::DARK_GRAY;

    pub fn random_select(&mut self) {
        let mut rng = thread_rng();

        let mode = self.bloons_config.modes.choose(&mut rng).expect("at least one mode should exist").clone();
        let map = self.bloons_config.maps.choose(&mut rng).expect("at least one map should exist").clone();
        // TODO wtf
        let hero = self.settings.active_heroes.iter().collect::<Vec<_>>().choose(&mut rng).cloned().cloned();
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
        let towers = self.bloons_config.get_towers_of_category(category).into_iter().collect::<Vec<_>>();
        towers
            .choose_multiple(rng, self.settings.get_amount(category) as usize)
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
                    FontId::new(36.0, FontFamily::Proportional)
                );
                ui.style_mut().text_styles.insert(
                    TextStyle::Body,
                    FontId::new(24.0, FontFamily::Proportional)
                );
                ui.style_mut().text_styles.insert(
                    TextStyle::Button,
                    FontId::new(24.0, FontFamily::Proportional)
                );

                ui.heading("Bloons Randomizer");

                Grid::new("monkey amount sliders").show(ui, |ui| {
                    ui.style_mut().text_styles.insert(
                        TextStyle::Body,
                        FontId::new(24.0, FontFamily::Proportional)
                    );

                    ui.label("Primary: ");
                    ui.add(egui::Slider::new(&mut self.settings.num_primary, 0..=self.bloons_config.num_towers_of_category(Primary)));
                    ui.end_row();
                    ui.label("Military: ");
                    ui.add(egui::Slider::new(&mut self.settings.num_military, 0..=self.bloons_config.num_towers_of_category(Military)));
                    ui.end_row();
                    ui.label("Magic: ");
                    ui.add(egui::Slider::new(&mut self.settings.num_magic, 0..=self.bloons_config.num_towers_of_category(Magic)));
                    ui.end_row();
                    ui.label("Support: ");
                    ui.add(egui::Slider::new(&mut self.settings.num_support, 0..=self.bloons_config.num_towers_of_category(Support)));
                });

                ui.collapsing("Include/Exclude Heroes", |ui| {
                    Grid::new("hero include exclude").show(ui, |ui| {
                        self.bloons_config.heroes
                            .iter()
                            .enumerate()
                            .for_each(|(i, hero)| {
                                let currently_selected = self.settings.active_heroes.contains(hero);
                                let tint = match currently_selected {
                                    true => Self::HERO_SELECTED_COLOR,
                                    false => Self::HERO_UNSELECTED_COLOR
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

                if ui.button("Randomize").clicked() {
                    self.random_select();
                }

                let selection = match &self.selection {
                    Some(s) => s,
                    None => return,
                };

                ui.horizontal(|ui| {
                    ui.add_sized([75.0, 75.0], Image::new(self.images.get_image(&selection.mode.icon)));
                    ui.label(&selection.mode.name);
                });
                ui.add(Image::new(self.images.get_image(&selection.map.icon)).max_size(Vec2::new(450.0, 300.0)));

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