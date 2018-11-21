use sfml::graphics::*;
use sfml::window::*;
use std::option::Option;


pub enum State {
    MENU,
    GAME,
}

pub trait Scene {
    fn update(&mut self, d: f32) -> Option<State>; //returns the new state if we need to change

    fn draw(&mut self, win: &mut RenderWindow);

    fn events(&mut self, evt: Event);
}

pub struct MenuScene {
    d: f64
}

impl MenuScene {
    pub fn new() -> MenuScene {
        MenuScene { d: 0.0 }
    }
}

impl Scene for MenuScene {
    fn update(&mut self, d: f32) -> Option<State> {
        println!("{}", d);

        None
    }

    fn draw(&mut self, win: &mut RenderWindow) {

    }

    fn events(&mut self, evt: Event) {

    }
}
