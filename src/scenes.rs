use sfml::graphics::*;
use sfml::window::*;
use std::option::Option;
use ui::UiButton;
use super::{WIN_WIDTH, WIN_HEIGHT};
use assets::AssetManager;

#[derive(Eq, PartialEq)]
pub enum State {
    Menu,
    Game,
    Exit,
}

pub trait Scene {
    fn update(&mut self, d: f32) -> Option<State>; //returns the new state if we need to change

    fn draw(&self, win: &mut RenderWindow);

    fn events(&mut self, evt: Event);
}

pub struct MenuScene<'a> {
    play_button: UiButton<'a>,
    exit_button: UiButton<'a>,
}

impl<'a> MenuScene<'a> {
    pub fn new(am: &'a AssetManager) -> MenuScene<'a> {
        let play_button = UiButton::new(am.get_font("consolas.ttf"))
            .bounds(WIN_WIDTH / 2.0 - 200.0, WIN_HEIGHT / 2.0, 400.0, 70.0)
            .color(Color::WHITE)
            .border_color(Color::BLACK)
            .border_thickness(3.0)
            .text("PLAY")
            .char_size(42)
            .text_color(Color::BLACK)
            .pack();

        let exit_button = UiButton::new(am.get_font("consolas.ttf"))
            .bounds(WIN_WIDTH / 2.0 - 200.0, WIN_HEIGHT / 2.0 + 120.0, 400.0, 70.0)
            .color(Color::WHITE)
            .border_color(Color::BLACK)
            .border_thickness(3.0)
            .text("EXIT")
            .char_size(42)
            .text_color(Color::BLACK)
            .pack();


        MenuScene { play_button, exit_button }
    }
}

impl<'a> Scene for MenuScene<'a> {
    fn update(&mut self, _d: f32) -> Option<State> {
        if self.play_button.clicked() {
            return Some(State::Game);
        }

        if self.exit_button.clicked() {
            return Some(State::Exit);
        }

        None
    }

    fn draw(&self, win: &mut RenderWindow) {
        self.play_button.draw(win);
        self.exit_button.draw(win);
    }

    fn events(&mut self, evt: Event) {
        self.play_button.event(evt);
        self.exit_button.event(evt);
    }
}

use actors::*;

pub struct GameScene<'a> {
    tower: Tower<'a>,
    score_text: Text<'a>,
    score: f32,
}

impl<'a> GameScene<'a> {
    pub fn new(am: &'a AssetManager) -> GameScene<'a> {
        let score_text = {
            let mut t = Text::new("0.0", am.get_font("consolas.ttf"), 10);
            t.set_fill_color(&Color::BLACK);

            t
        };

        GameScene {
            tower: Tower::new(am),
            score_text,
            score: 0.0,
        }
    }
}

impl<'a> Scene for GameScene<'a> {
    fn update(&mut self, d: f32) -> Option<State> {
        self.tower.update(d);

        self.score += self.tower.num_bullets() as f32 * 7.98 * d;
        self.score_text.set_string(format!("{}", self.score).as_str());

        if self.tower.dead {
            return Some(State::Menu);
        }

        None
    }

    fn draw(&self, win: &mut RenderWindow) {
        self.tower.draw(win);
        win.draw(&self.score_text);
    }

    fn events(&mut self, evt: Event) {
        self.tower.events(evt);
    }
}
