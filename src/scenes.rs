use sfml::graphics::*;
use sfml::window::*;
use std::option::Option;
use ui::UiButton;
use super::{WIN_WIDTH, WIN_HEIGHT};
use assets::AssetManager;
use sfml::system::Vector2f;

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
        let play_button = UiButton::new(am.get_font("font.ttf"))
            .bounds(WIN_WIDTH / 2.0 - 200.0, WIN_HEIGHT / 2.0, 400.0, 70.0)
            .color(Color::WHITE)
            .border_color(Color::BLACK)
            .border_thickness(3.0)
            .text("PLAY")
            .char_size(42)
            .text_color(Color::BLACK)
            .pack();

        let exit_button = UiButton::new(am.get_font("font.ttf"))
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
    score_len: usize,
}

impl<'a> GameScene<'a> {
    pub fn new(am: &'a AssetManager) -> GameScene<'a> {
        let score_text = {
            let mut t = Text::new("0.0", am.get_font("font.ttf"), 24);
            t.set_fill_color(&Color::BLACK);
            t.set_position(Vector2f::new(WIN_WIDTH / 2.0, 10.0));

            t
        };

        GameScene {
            tower: Tower::new(am),
            score_text,
            score: 0.0,
            score_len: 0,
        }
    }
}

impl<'a> Scene for GameScene<'a> {
    fn update(&mut self, d: f32) -> Option<State> {
        self.tower.update(d);
        self.score += self.tower.num_bullets() as f32 * 4.0 / (self.tower.num_bullets() as f32 / 8.0 + 1.0) * d;

        let ss = format!("Score: {:.0}", self.score);
        self.score_text.set_string(ss.as_str());

        if ss.len() != self.score_len {
            self.score_len = ss.len();

            let p = {
                let mut v = Vector2f::new(WIN_WIDTH / 2.0, 10.0);
                v.x -= (self.score_text.local_bounds().width / 2.0).trunc();
                v.x = v.x.trunc();

                v
            };

            self.score_text.set_position(p);
        }

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
