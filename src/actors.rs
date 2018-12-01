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

const TOWER_INNER: f32 = 90.0;
const TOWER_OUTER: f32 = 250.0;

pub struct Tower<'a> {
    sprite: Sprite<'a>,
    teleport_circle: CircleShape<'a>,
    position: Vector2f,
    rotation: f32,
    bullet_sprite: Sprite<'a>,
    bullets: Vec<Bullet<'a>>,
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

        let teleport_circle = {
            let mut c = CircleShape::new(TOWER_INNER, 36);
            c.set_position(position);
            c.set_outline_thickness(TOWER_OUTER - TOWER_INNER);
            c.set_origin(Vector2f::new(TOWER_INNER, TOWER_INNER));
            c.set_fill_color(&Color::TRANSPARENT);
            c.set_outline_color(&Color::rgba(255, 255, 255, 80));

            c
        };

        let bullet_sprite = {
            let mut s = Sprite::new();
            let t = am.get_texture("bullet.png");
            s.set_texture(t, true);
            s.set_position(position);
            s.set_origin(Vector2f { x: t.size().x as f32 / 2.0, y: t.size().y as f32 / 2.0 });

            s
        };


        Tower {
            sprite,
            teleport_circle,
            position,
            rotation: 0.0,
            bullet_sprite,
            bullets: Vec::new(),
        }
    }
}

impl<'a> Actor for Tower<'a> {
    fn update(&mut self, d: f32) {
        self.sprite.set_rotation(self.rotation);
        self.sprite.set_position(self.position);
        self.teleport_circle.set_position(self.position);

        for b in &mut self.bullets {
            b.update(d);
        }
    }

    fn draw(&self, win: &mut RenderWindow) {
        for b in &self.bullets {
            b.draw(win);
        }

        win.draw(&self.sprite);
        win.draw(&self.teleport_circle);
    }

    fn events(&mut self, evt: Event) {
        match evt {
            Event::MouseButtonPressed { button: Button::Left, x, y } => {
                self.position += {
                    let mut m = Vector2f::new(x as f32 - self.position.x, y as f32 - self.position.y);
                    let mut l = f32::sqrt(m.x.powi(2) + m.y.powi(2));

                    if l != 0.0 {
                        m.x /= l;
                        m.y /= l;
                        l = f32::min(TOWER_OUTER, f32::max(l, TOWER_INNER));
                        m.x *= l;
                        m.y *= l;

                        self.bullets.push(Bullet::new(self.bullet_sprite.clone(), self.position, self.rotation));
                    }

                    m
                }
            }
            Event::MouseButtonReleased { button: Button::Left, x, y } => {}
            Event::MouseMoved { x, y } => {
                self.rotation = (y as f32 - self.position.y).atan2(x as f32 - self.position.x).to_degrees();
            }

            _ => {}
        }
    }
}

const BULLET_SPEED: f32 = 100.0;

pub struct Bullet<'a> {
    sprite: Sprite<'a>,
    position: Vector2f,
    rotation: f32,
}

impl<'a> Bullet<'a> {
    pub fn new(spr: Sprite<'a>, pos: Vector2f, rot: f32) -> Bullet<'a> {
        let mut b = Bullet {
            sprite: spr,
            position: pos,
            rotation: rot,
        };

        b.update(1.0 / 100.0);

        b
    }
}

impl<'a> Actor for Bullet<'a> {
    fn update(&mut self, d: f32) {
        if self.position.x <= 0.0 || self.position.x >= WIN_WIDTH {
            self.rotation = -self.rotation + 180.0;
        }

        if self.position.y <= 0.0 || self.position.y >= WIN_HEIGHT {
            self.rotation = -self.rotation;
        }

        self.position.x += self.rotation.to_radians().cos() * BULLET_SPEED * d;
        self.position.y += self.rotation.to_radians().sin() * BULLET_SPEED * d;

        self.sprite.set_rotation(self.rotation);
        self.sprite.set_position(self.position);
    }

    fn draw(&self, win: &mut RenderWindow) {
        win.draw(&self.sprite);
    }

    fn events(&mut self, evt: Event) {
        unimplemented!();
    }
}
