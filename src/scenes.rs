use std::fs::*;
use std::io::*;
use std::option::Option;

use sfml::graphics::*;
use sfml::system::Vector2f;
use sfml::window::*;

use crate::actors::*;
use crate::assets::AssetManager;
use crate::ui::UiButton;

use super::{WIN_HEIGHT, WIN_WIDTH};

#[derive(PartialEq)]
pub enum State {
    Menu(Option<f32>),
    Game,
    Exit,
}

pub trait Scene {
    fn update(&mut self, d: f32) -> Option<State>; //returns the new state if we need to change

    fn draw(&self, win: &mut RenderWindow);

    fn events(&mut self, evt: Event);
}

pub struct MenuScene<'a> {
    title_text: Text<'a>,
    help_text: Text<'a>,
    highscore_text: Text<'a>,
    play_button: UiButton<'a>,
    exit_button: UiButton<'a>,
    highscore: f32,
}

impl<'a> MenuScene<'a> {
    pub fn new(am: &'a AssetManager, pre_highscore: Option<f32>) -> MenuScene<'a> {
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

        let title_text = {
            let mut t = Text::new("Moving Tower", am.get_font("font.ttf"), 56);
            t.set_fill_color(&Color::BLACK);

            let p = {
                let mut v = Vector2f::new(WIN_WIDTH / 2.0, 15.0);
                v.x -= (t.local_bounds().width / 2.0).trunc();
                v.x = v.x.trunc();

                v
            };

            t.set_position(p);

            t
        };

        let help_text = {
            let mut t = Text::new("Click to teleport within the ring\nand don't get hit by a bullet", am.get_font("font.ttf"), 19);
            t.set_fill_color(&Color::BLACK);

            let p = {
                let mut v = Vector2f::new(WIN_WIDTH / 2.0, 120.0);
                v.x -= (t.local_bounds().width / 2.0).trunc();
                v.x = v.x.trunc();

                v
            };

            t.set_position(p);

            t
        };

        let mut highscore = load_highscore();
        match pre_highscore {
            Some(hs) => {
                highscore = hs.max(highscore);
                save_highscore(highscore);
            }

            _ => {}
        }

        let highscore_text = {
            let mut t = Text::new(format!("Highscore: {:.0}", highscore).as_str(), am.get_font("font.ttf"), 19);
            t.set_fill_color(&Color::BLACK);

            let p = {
                let mut v = Vector2f::new(WIN_WIDTH / 2.0, 200.0);
                v.x -= (t.local_bounds().width / 2.0).trunc();
                v.x = v.x.trunc();

                v
            };

            t.set_position(p);

            t
        };

        MenuScene {
            title_text,
            help_text,
            highscore_text,
            play_button,
            exit_button,
            highscore,
        }
    }
}

fn load_highscore() -> f32 {
    let mut hs: f32 = 0.0;

    match File::open("highscore.txt") {
        Ok(mut f) => {
            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => {
                    match s.parse::<f32>() {
                        Ok(o) => { hs = o }
                        _ => {}
                    }
                }

                _ => {}
            }
        }

        _ => {}
    }

    hs
}

fn save_highscore(hs: f32) {
    match OpenOptions::new().write(true).open("highscore.txt") {
        Ok(mut f) => {
            f.write_all(format!("{}", hs).as_bytes());
            f.flush();
        }

        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                match File::create("highscore.txt") {
                    Ok(mut f) => {
                        f.write(format!("{}", hs).as_bytes());
                        f.flush();
                    }

                    _ => {
                        println!("Couldn't save!");
                    }
                }
            }

            _ => {
                println!("Couldn't save!");
            }
        }
    }
}

impl<'a> Scene for MenuScene<'a> {
    fn update(&mut self, _d: f32) -> Option<State> {
        if self.play_button.clicked() {
            return Some(State::Game);
        }

        if self.exit_button.clicked() {
            save_highscore(self.highscore);
            return Some(State::Exit);
        }

        None
    }

    fn draw(&self, win: &mut RenderWindow) {
        self.play_button.draw(win);
        self.exit_button.draw(win);
        win.draw(&self.title_text);
        win.draw(&self.help_text);
        win.draw(&self.highscore_text);
    }

    fn events(&mut self, evt: Event) {
        self.play_button.event(evt);
        self.exit_button.event(evt);
    }
}

pub struct GameScene<'a> {
    tower: Tower<'a>,
    score_text: Text<'a>,
    score: f32,
    score_len: usize,
}

impl<'a> GameScene<'a> {
    pub fn new(am: &'a AssetManager) -> GameScene<'a> {
        let score_text = {
            let mut t = Text::new("0.0", am.get_font("font.ttf"), 20);
            t.set_fill_color(&Color::BLACK);
            t.set_position(Vector2f::new(WIN_WIDTH / 2.0, 5.0));

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
                let mut v = Vector2f::new(WIN_WIDTH / 2.0, 5.0);
                v.x -= (self.score_text.local_bounds().width / 2.0).trunc();
                v.x = v.x.trunc();

                v
            };

            self.score_text.set_position(p);
        }

        if self.tower.dead {
            return Some(State::Menu(Some(self.score)));
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
