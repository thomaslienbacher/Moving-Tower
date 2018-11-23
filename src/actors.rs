use sfml::graphics::Sprite;

pub struct Tower<'a> {
    sprite: Sprite<'a>
}

impl<'a> Tower<'a> {
    pub fn new() -> Tower<'a> {
        let sprite = {
            let mut s = Sprite::new();

            s
        };
        Tower { sprite }
    }
}