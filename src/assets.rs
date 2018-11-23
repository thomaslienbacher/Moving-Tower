use std::collections::HashMap;
use sfml::graphics::*;
use utils::get_path;

pub struct AssetManager {
    res_path: String,
    fonts: HashMap<String, Font>,
    textures: HashMap<String, Texture>,
}

impl AssetManager {
    pub fn new(res_path: &str) -> AssetManager {
        AssetManager {
            res_path: res_path.to_string(),
            fonts: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn load_font(&mut self, font: &str) {
        let f = {
            let path = get_path((self.res_path.clone() + font).as_str());
            if let Some(f) = Font::from_file(path.as_str()) {
                f
            } else {
                panic!("Couldn't load font: {}", path);
            }
        };

        self.fonts.insert(font.to_string(), f);
    }

    pub fn load_texture(&mut self, texture: &str) {
        let t = {
            let path = get_path((self.res_path.clone() + texture).as_str());
            if let Some(mut t) = Texture::from_file(path.as_str()) {
                t.set_smooth(true);

                t
            } else {
                panic!("Couldn't load texture: {}", path);
            }
        };

        self.textures.insert(texture.to_string(), t);
    }

    pub fn get_font(&self, font: &str) -> &Font {
        if let Some(f) = self.fonts.get(font) {
            return f;
        } else {
            panic!("Font not loaded: {}", font);
        }
    }

    pub fn get_texture(&self, texture: &str) -> &Texture {
        if let Some(t) = self.textures.get(texture) {
            return t;
        } else {
            panic!("Texture not loaded: {}", texture);
        }
    }
}

