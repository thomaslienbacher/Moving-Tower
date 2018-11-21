use sfml::graphics::*;
use sfml::window::*;
use std::option::Option;
use ui::UiButton;
use utils::get_path;

pub enum State {
    MENU,
    GAME,
}

pub trait Scene {
    fn update(&mut self, d: f32) -> Option<State>; //returns the new state if we need to change

    fn draw(&self, win: &mut RenderWindow);

    fn events(&mut self, evt: Event);
}

pub struct MenuScene<'a> {
    play_button: UiButton<'a>
}

impl<'a> MenuScene<'a> {
    pub fn new(font: &'a Font) -> MenuScene<'a> {
        let pb = UiButton::new(&font)
            .size(100, 100, 200, 50)
            .color(&Color::WHITE)
            .border_color(&Color::BLACK)
            .border_thickness(3.0)
            .text("PLAY")
            .char_size(32)
            .text_color(&Color::BLACK)
            .pack();


        MenuScene { play_button: pb }
    }
}

impl<'a> Scene for MenuScene<'a> {
    fn update(&mut self, _d: f32) -> Option<State> {
        if self.play_button.clicked() {
            println!("play button was clicked!");
        }

        None
    }

    fn draw(&self, win: &mut RenderWindow) {
        self.play_button.draw(win);
    }

    fn events(&mut self, evt: Event) {
        self.play_button.event(evt);
    }
}
