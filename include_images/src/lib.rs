use proc_macro::TokenStream;
use std::fs::read_dir;
use std::io;
use std::path::Path;

/// Include all images in the assets path into the binary to make them available all the time.
#[proc_macro]
pub fn include_images(_item: TokenStream) -> TokenStream {
    let image_paths = load_image_paths();
    let mut expression = "{let mut path_image_map = std::collections::HashMap::new();".to_string();

    for path in image_paths {
        let simple_path = path.replace("../assets/", "");
        expression += format!("path_image_map.insert(\"{simple_path}\", egui::include_image!(\"{path}\"));").as_str();
    }

    expression += "path_image_map}";
    expression.parse().unwrap()
}

fn load_image_paths() -> Vec<String> {
    load_image_paths_recursive(Path::new("./assets/")).expect("the assets folder should exist")
}

fn load_image_paths_recursive(path: &Path) -> io::Result<Vec<String>> {
    let mut files = vec![];

    if path.is_dir() {
        for entry in read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                files.extend(load_image_paths_recursive(&path)?.into_iter());
            } else {
                let path_str = path
                    .to_str()
                    .unwrap()
                    .replace('\\', "/")
                    .replace("./assets/", "../assets/")
                    .to_string();

                if path_str.ends_with(".webp") {
                    files.push(path_str);
                }
            }
        }
    }

    Ok(files)
}