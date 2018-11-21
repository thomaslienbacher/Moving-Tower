extern crate sfml;

mod utils;
mod scenes;
mod ui;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;
use scenes::{MenuScene, Scene, State};

const WIN_SIZE: (u32, u32) = (800, 600);

fn main() {
    let mut window = RenderWindow::new(
        VideoMode::new(WIN_SIZE.0, WIN_SIZE.1, 8),
        "Moving Tower",
        Style::CLOSE,
        &Default::default(),
    );

    window.set_framerate_limit(60);

    let mut clock = Clock::default();
    let mut gamestate = State::MENU;

    //scenes
    let mut menuscene =  MenuScene::new();
    let mut curscene = &mut menuscene as &mut Scene;

    while window.is_open() {
        let delta = clock.restart().as_seconds();
        curscene.update(delta);

        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => { window.close() }
                Event::
                KeyPressed { code, .. } => {
                    match code {
                        Key::Escape => { window.close() }
                        _ => {}
                    }
                }
                _ => {}
            }

            curscene.events(ev);
        }

        window.clear(&Color::from(0x1AA177));

        curscene.draw(&mut window);

        window.display();
    }
}