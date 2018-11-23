use std::collections::HashMap;
use sfml::graphics::*;
use utils::get_path;

pub struct AssetManager {
    fonts: HashMap<String, Font>,
    textures: HashMap<String, Texture>,
}

impl AssetManager {
    pub fn new() -> AssetManager {
        AssetManager {
            fonts: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn load_font(&mut self, font: &str) {
        let f = {
            if let Some(f) = Font::from_file(get_path(font).as_str()) {
                f
            } else {
                panic!("Couldn't load font: {}", font);
            }
        };

        self.fonts.insert(font.to_string(), f);
    }

    pub fn load_texture(&mut self, texture: &str) {
        let t = {
            if let Some(t) = Texture::from_file(get_path(texture).as_str()) {
                t
            } else {
                panic!("Couldn't load texture: {}", texture);
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
            panic!("Font not loaded: {}", texture);
        }
    }
}

