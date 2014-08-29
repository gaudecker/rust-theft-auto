extern crate piston;
extern crate sdl2_game_window;

use sdl2_game_window::GameWindowSDL2 as Window;
use piston::gfx;
use piston::gfx::{Device, DeviceHelper};
use piston::{cam, GameWindow};

use map::{Map};

mod map;

fn main() {
    let mut window = Window::new(
        piston::shader_version::opengl::OpenGL_3_2, 
        piston::GameWindowSettings {
            title: "gta".to_string(),
            size: [960, 540],
            fullscreen: false,
            exit_on_esc: true
        }
    );
    window.capture_cursor(true);

    let map = match Map::from_file("data/nyc.cmp") {
        Err(why) => fail!("Could not load map: {}", why.desc),
        Ok(map) => map
    };

    let (mut device, frame) = window.gfx();
    let state = gfx::DrawState::new().depth(gfx::state::LessEqual, true);

    let mut graphics = gfx::Graphics::new(device);

    // let model = piston::vecmath::mat4_id();
    let projection = cam::CameraPerspective {
        fov: 90.0f32,
        near_clip: 0.1,
        far_clip: 1000.0,
        aspect_ratio: 1.0
    }.projection();

    let mut first_person = cam::FirstPerson::new(
        [0.5f32, 0.5, 4.0],
        cam::FirstPersonSettings::keyboard_wasd()
    );

    let mut game_iter = piston::GameIterator::new(
        &mut window,
        &piston::GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60
        }
    );

    for e in game_iter {
        match e {
            piston::Render(_args) => {
                graphics.clear(
                    gfx::ClearData {
                        color: Some([0.0, 0.0, 0.0, 1.0]),
                        depth: Some(1.0),
                        stencil: None,
                    },
                    &frame
                );
                graphics.end_frame();
            },
            piston::Update(args) => first_person.update(args.dt),
            piston::Input(e) => first_person.input(&e),
        }
    }
}
