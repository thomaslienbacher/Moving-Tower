use sfml::graphics::*;
use sfml::system::Vector2f;
use sfml::window::Event;
use sfml::window::mouse::Button;

pub struct UiButton<'a> {
    shape: RectangleShape<'a>,
    text: Text<'a>,
    rect: FloatRect,
    clicked: bool,
    down: bool,
    fill_color: Color,
    border_color: Color,
}

const DOWN_SCALE: f32 = 0.97;
const DARKENING_SCALE: f32 = 0.9;

impl<'a> UiButton<'a> {
    pub fn new(font: &'a Font) -> UiButton<'a> {
        UiButton {
            shape: RectangleShape::new(),
            text: Text::new("", font, 16),
            rect: FloatRect::default(),
            clicked: false,
            down: false,
            fill_color: Color::WHITE,
            border_color: Color::BLACK,
        }
    }

    pub fn bounds(mut self, x: f32, y: f32, w: f32, h: f32) -> Self {
        self.rect = FloatRect::new(x, y, w, h);
        self.shape.set_size(Vector2f { x: w, y: h });
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn border_thickness(mut self, thickness: f32) -> Self {
        self.shape.set_outline_thickness(thickness);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text.set_string(text);
        self
    }

    pub fn char_size(mut self, size: u32) -> Self {
        self.text.set_character_size(size);
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text.set_fill_color(&color);
        self
    }

    pub fn pack(mut self) -> Self {
        self.shape.set_fill_color(&self.fill_color);
        self.shape.set_outline_color(&self.border_color);

        let pos = {
            let x = self.rect.left
                + self.rect.width / 2.0
                - self.text.global_bounds().width / 2.0;

            let y = self.rect.top
                + self.rect.height / 2.0
                - self.text.global_bounds().height;//TODO: divide by 2 ??

            Vector2f::new(x.round(), y.round())
        };
        self.text.set_position(pos);

        let org = {
            let x = self.rect.width / 2.0;
            let y = self.rect.height / 2.0;

            Vector2f::new(x, y)
        };
        self.shape.set_origin(org);
        self.shape.set_position(Vector2f::new(self.rect.left + org.x, self.rect.top + org.y));

        self
    }

    //TODO: add more down state features
    pub fn draw(&self, win: &mut RenderWindow) {
        win.draw(&self.shape);
        win.draw(&self.text);
    }

    pub fn clicked(&mut self) -> bool {
        if self.clicked {
            self.clicked = false;
            return true;
        }

        false
    }

    pub fn event(&mut self, evt: Event) {
        match evt {
            Event::MouseButtonPressed { button: Button::Left, x, y } => {
                if self.rect.contains2(x as f32, y as f32) {
                    self.shape.set_scale(Vector2f::new(DOWN_SCALE, DOWN_SCALE));
                    self.down = true;
                }
            }
            Event::MouseButtonReleased { button: Button::Left, x, y } => {
                if self.down && self.rect.contains2(x as f32, y as f32) {
                    self.clicked = true;
                }

                self.shape.set_scale(Vector2f::new(1.0, 1.0));
                self.down = false;
            }
            Event::MouseMoved { x, y } => {
                if self.rect.contains2(x as f32, y as f32) {
                    let fc = {
                        let mut c = self.fill_color.clone();
                        c.r = (c.r as f32 * DARKENING_SCALE) as u8;
                        c.g = (c.g as f32 * DARKENING_SCALE) as u8;
                        c.b = (c.b as f32 * DARKENING_SCALE) as u8;

                        c
                    };

                    let bc = {
                        let mut c = self.border_color.clone();
                        c.r = (c.r as f32 * DARKENING_SCALE) as u8;
                        c.g = (c.g as f32 * DARKENING_SCALE) as u8;
                        c.b = (c.b as f32 * DARKENING_SCALE) as u8;

                        c
                    };

                    self.shape.set_fill_color(&fc);
                    self.shape.set_outline_color(&bc);
                } else {
                    self.shape.set_fill_color(&self.fill_color);
                    self.shape.set_outline_color(&self.border_color);
                }
            }

            _ => {}
        }
    }
}