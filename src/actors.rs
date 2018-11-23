use sfml::graphics::*;
use sfml::window::*;
use sfml::system::Vector2f;
use super::{WIN_WIDTH, WIN_HEIGHT};
use AssetManager;
use sfml::window::mouse::Button;

pub trait Actor {
    fn update(&mut self, d: f32);

    fn draw(&self, win: &mut RenderWindow);

    fn events(&mut self, evt: Event);
}

pub struct Tower<'a> {
    sprite: Sprite<'a>,
    position: Vector2f,
    rotation: f32,
}

impl<'a> Tower<'a> {
    pub fn new(am: &'a AssetManager) -> Tower<'a> {
        let position = Vector2f::new(WIN_WIDTH / 2.0, WIN_HEIGHT / 2.0);

        let sprite = {
            let mut s = Sprite::new();
            let t = am.get_texture("tower.png");

            s.set_texture(t, true);
            s.set_position(position);
            s.set_origin(Vector2f { x: t.size().x as f32 / 2.0, y: t.size().y as f32 / 2.0 });

            s
        };


        Tower {
            sprite,
            position,
            rotation: 0.0,
        }
    }
}

impl<'a> Actor for Tower<'a> {
    fn update(&mut self, d: f32) {}

    fn draw(&self, win: &mut RenderWindow) {
        win.draw(&self.sprite);
    }

    fn events(&mut self, evt: Event) {
        match evt {
            Event::MouseButtonPressed { button: Button::Left, x, y } => {

            }
            Event::MouseButtonReleased { button: Button::Left, x, y } => {

            }
            Event::MouseMoved { x, y } => {
                self.rotation = (y as f32 - self.position.y).atan2(x as f32 - self.position.x).to_degrees();
                self.sprite.set_rotation(self.rotation);
            }

            _ => {}
        }
    }
}