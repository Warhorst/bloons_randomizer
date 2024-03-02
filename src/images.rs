use std::collections::HashMap;
use egui::ImageSource;
use include_images::include_images;

pub struct Images<'a> {
    path_image_map: HashMap<&'static str, ImageSource<'a>>
}

impl <'a> Default for Images<'a> {
    fn default() -> Self {
        Images {
            path_image_map: include_images!()
        }
    }
}

impl<'a> Images<'a> {
    pub fn get_image(&self, path: &str) -> ImageSource {
        self.path_image_map.get(path).unwrap().clone()
    }
}