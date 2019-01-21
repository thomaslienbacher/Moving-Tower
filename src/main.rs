#![windows_subsystem = "windows"]

extern crate sfml;

use std::boxed::Box;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;

use crate::assets::AssetManager;
use crate::scenes::*;
use crate::utils::get_path;

mod utils;
mod scenes;
mod ui;
mod assets;
mod actors;

const WIN_SIZE: (u32, u32) = (1000, 600);
const WIN_WIDTH: f32 = WIN_SIZE.0 as f32;
const WIN_HEIGHT: f32 = WIN_SIZE.1 as f32;

fn main() {
    let mut settings = ContextSettings::default();
    settings.antialiasing_level = 4;

    let mut window = RenderWindow::new(
        VideoMode::new(WIN_SIZE.0, WIN_SIZE.1, 8),
        "Moving Tower",
        Style::CLOSE,
        &settings,
    );

    window.set_framerate_limit(60);

    let icon = {
        let path = get_path("resources/icon.png");
        if let Some(t) = Image::from_file(path.as_str()) {
            t
        } else {
            panic!("Couldn't load icon: {}", path);
        }
    };

    window.set_icon(icon.size().x, icon.size().y, icon.pixel_data());

    let mut asset_manager = AssetManager::new("resources/");
    asset_manager.load_font("font.ttf");
    asset_manager.load_texture("tower.png");
    asset_manager.load_texture("bullet.png");

    let mut clock = Clock::default();
    let mut curscene: Box<Scene> = Box::new(MenuScene::new(&asset_manager, None)) as Box<Scene>;

    while window.is_open() {
        let delta = clock.restart().as_seconds();

        if let Some(s) = curscene.update(delta) {
            match s {
                State::Menu(hs) => {
                    curscene = Box::new(MenuScene::new(&asset_manager, hs)) as Box<Scene>;
                }
                State::Game => {
                    curscene = Box::new(GameScene::new(&asset_manager)) as Box<Scene>;
                }
                State::Exit => {
                    window.close()
                }
            }
        }

        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => { window.close() }
                Event::KeyPressed {
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

        window.clear(&Color::from(0x777777FF));

        curscene.draw(&mut window);

        window.display();
    }
}