use eframe::{App, Error, Frame, run_native};
use egui::{CentralPanel, Context, Grid, Image, Vec2};
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

fn main() -> Result<(), Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_fullscreen(true),
        ..Default::default()
    };

    run_native(
        "Bloons Randomizer",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<BloonsRandomizerApp<'_>>::default()
        }),
    )
}

#[derive(Default)]
struct BloonsRandomizerApp<'a> {
    bloons_config: BloonsConfig,
    images: Images<'a>,
    settings: Settings,
    selection: Option<Selection>,
}

impl<'a> BloonsRandomizerApp<'a> {
    pub fn random_select(&mut self) {
        let mut rng = thread_rng();

        let mode = self.bloons_config.modes.choose(&mut rng).expect("at least one mode should exist").clone();
        let map = self.bloons_config.maps.choose(&mut rng).expect("at least one map should exist").clone();
        let hero = self.bloons_config.heroes.choose(&mut rng).expect("at least one hero should exist").clone();
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
            ctx.set_pixels_per_point(1.5);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Bloons Randomizer");
                ui.horizontal(|ui| {
                    ui.label("Primary: ");
                    ui.add(egui::Slider::new(&mut self.settings.num_primary, 0..=self.bloons_config.num_towers_of_category(Primary)));
                });
                ui.horizontal(|ui| {
                    ui.label("Military: ");
                    ui.add(egui::Slider::new(&mut self.settings.num_military, 0..=self.bloons_config.num_towers_of_category(Military)));
                });
                ui.horizontal(|ui| {
                    ui.label("Magic: ");
                    ui.add(egui::Slider::new(&mut self.settings.num_magic, 0..=self.bloons_config.num_towers_of_category(Magic)));
                });
                ui.horizontal(|ui| {
                    ui.label("Support: ");
                    ui.add(egui::Slider::new(&mut self.settings.num_support, 0..=self.bloons_config.num_towers_of_category(Support)));
                });

                if ui.button("Randomize").clicked() {
                    self.random_select();
                }

                let selection = match &self.selection {
                    Some(s) => s,
                    None => return,
                };

                ui.horizontal(|ui| {
                    ui.add_sized([50.0, 50.0], Image::new(self.images.get_image(&selection.mode.icon)));
                    ui.label(&selection.mode.name);
                });
                ui.add(Image::new(self.images.get_image(&selection.map.icon)).max_size(Vec2::new(300.0, 200.0)));
                ui.add(Image::new(self.images.get_image(&selection.hero.icon)).max_size(Vec2::new(200.0, 100.0)));
                Grid::new("grid").show(ui, |ui| {
                    selection.towers
                        .iter()
                        .enumerate()
                        .for_each(|(i, tower)| {
                            ui.add_sized([50.0, 50.0], Image::new(self.images.get_image(&tower.icon)));
                            if (i + 1) % 5 == 0 {
                                ui.end_row();
                            }
                        })
                });
            });
        });
    }
}