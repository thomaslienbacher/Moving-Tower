extern crate sfml;

mod utils;
mod scenes;
mod ui;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;
use scenes::{MenuScene, Scene, State};
use utils::get_path;

const WIN_SIZE: (u32, u32) = (800, 600);

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
    let _gamestate = State::MENU;

    //scenes
    let mut menuscene = MenuScene::new(&font);
    let curscene = &mut menuscene as &mut Scene;

    while window.is_open() {
        let delta = clock.restart().as_seconds();
        curscene.update(delta);

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