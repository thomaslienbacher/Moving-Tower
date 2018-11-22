extern crate sfml;

mod utils;
mod scenes;
mod ui;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;
use scenes::*;
use utils::get_path;
use std::boxed::Box;

const WIN_SIZE: (u32, u32) = (1000, 600);
const WIN_WIDTH: f32 = WIN_SIZE.0 as f32;
const WIN_HEIGHT: f32 = WIN_SIZE.1 as f32;

fn main() {
    let mut window = RenderWindow::new(
        VideoMode::new(WIN_SIZE.0, WIN_SIZE.1, 8),
        "Moving Tower",
        Style::CLOSE,
        &Default::default(),
    );

    window.set_framerate_limit(60);

    let font = {
        if let Some(f) = Font::from_file(get_path("resources/consolas.ttf").as_str()) {
            f
        } else {
            panic!("Couldn't load font");
        }
    };

    let mut clock = Clock::default();
    let mut curscene: Box<Scene> = Box::new(MenuScene::new(&font)) as Box<Scene>;

    while window.is_open() {
        let delta = clock.restart().as_seconds();

        if let Some(s) = curscene.update(delta) {
            match s {
                State::Menu => {
                    curscene = Box::new(MenuScene::new(&font)) as Box<Scene>;
                }
                State::Game => {
                    curscene = Box::new(GameScene::new()) as Box<Scene>;
                }
                State::Exit => {
                    window.close()
                }
            }
        }

        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => { window.close() }
                Event::
                KeyPressed {
                    code, ..
                } => {
                    match code {
                        Key::Escape => { window.close() }
                        _ => {}
                    }
                }
                _ => {}
            }

            curscene.events(ev);
        }

        window.clear(&Color::from(0x99CC77));

        curscene.draw(&mut window);

        window.display();
    }
}