extern crate piston;
extern crate glfw_game_window;
extern crate gl;

use Window = glfw_game_window::GameWindowGLFW;
use piston::{
    GameIterator,
    GameIteratorSettings,
    GameWindowSettings,
    Render
};

use map::{Map};

mod map;

fn main() {
    let ref mut window = Window::new(GameWindowSettings {
        title: "gta".to_string(),
        size: [960, 540],
        fullscreen: false,
        exit_on_esc: true
    });

    let map = match Map::from_file("data/nyc.cmp") {
        Err(why) => fail!("could not load map: {}", why.desc),
        Ok(map) => map
    };

    let mut game_iter = GameIterator::new(window, &GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    });

    for e in game_iter {
        match e {
            Render(_args) => {
            },
            _ => {},
        }
    }
}
