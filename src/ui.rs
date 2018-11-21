use sfml::window::Event;
use sfml::graphics::*;
use sfml::window::mouse::Button;
use sfml::system::Vector2f;


pub struct UiButton<'a> {
    shape: RectangleShape<'a>,
    text: Text<'a>,
    rect: IntRect,
    clicked: bool,
    down: bool, //whether the button is down or not
}

impl<'a> UiButton<'a> {
    pub fn new(font: &'a Font) -> UiButton<'a> {
        UiButton {
            shape: RectangleShape::new(),
            text: Text::new("", font, 16),
            rect: IntRect::default(),
            clicked: false,
            down: false,
        }
    }

    pub fn size(mut self, x: i32, y: i32, w: i32, h: i32) -> Self {
        self.rect = IntRect::new(x, y, w, h);
        self.shape.set_size(Vector2f { x: w as f32, y: h as f32 });
        self
    }

    pub fn color(mut self, color: &Color) -> Self {
        self.shape.set_fill_color(color);
        self
    }

    pub fn border_color(mut self, color: &Color) -> Self {
        self.shape.set_outline_color(color);
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

    pub fn text_color(mut self, color: &Color) -> Self {
        self.text.set_fill_color(color);
        self
    }

    pub fn pack(mut self) -> Self {
        let pos = {
            let x = self.rect.left as f32
                + self.rect.width as f32 / 2.0
                - self.text.global_bounds().width / 2.0;

            let y = self.rect.top as f32
                + self.rect.height as f32 / 2.0
                - self.text.global_bounds().height;//TODO: should we divide by 2 ??

            Vector2f::new(x, y)
        };
        self.text.set_position(pos);

        let org = {
            let x = self.rect.width as f32 / 2.0;
            let y = self.rect.height as f32 / 2.0;

            Vector2f::new(x, y)
        };
        self.shape.set_origin(org);
        self.shape.set_position(Vector2f::new(self.rect.left as f32 + org.x, self.rect.top as f32 + org.y,));

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
                if self.rect.contains2(x, y) {
                    self.down = true;
                    self.shape.set_scale(Vector2f::new(0.95, 0.95));//TODO: constify
                }
            }
            Event::MouseButtonReleased { button: Button::Left, x, y } => {
                if self.down && self.rect.contains2(x, y) {
                    self.clicked = true;
                }

                self.down = false;
                self.shape.set_scale(Vector2f::new(1.0, 1.0));
            }

            _ => {}
        }
    }
}