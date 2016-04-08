extern crate sdl2;
extern crate tetrs;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use tetrs::engine::Engine;
use tetrs::controller::Action;
use tetrs::block::Rotation;
use tetrs::randomizer::Randomizer;
use std::thread;
use std::time::Duration;

static KEYMAP: [(Scancode, Action); 9] = [
    (Scancode::Left,  Action::MoveLeft),
    (Scancode::Right, Action::MoveRight),
    (Scancode::Down,  Action::MoveDown),
    (Scancode::Space, Action::HardDrop),
    (Scancode::Z,     Action::RotateLeft),
    (Scancode::X,     Action::RotateRight),
    (Scancode::C,     Action::Hold),
    (Scancode::Q,     Action::Quit),
    (Scancode::Escape,Action::Quit),
];

fn gather_input(engine: &mut Engine, pump: &mut sdl2::EventPump) {
    engine.controller.deactivate_all();

    for &(scancode, action) in KEYMAP.iter() {
        if pump.keyboard_state().is_scancode_pressed(scancode) {
            engine.controller.activate(action);
        }
    }

    // Handle Window manager close
    for event in pump.poll_iter() {
        match event {
            Event::Quit{..} => engine.running = false,
            _ => ()
        }
    }
}

fn main() {

    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window = match video_ctx.window("tetrs", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err)     => panic!("failed to create renderer: {}", err)
    };

    let mut events = ctx.event_pump().unwrap();

    let mut engine = Engine::new();

    while engine.running {
        gather_input(&mut engine, &mut events);

        engine.update();

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        // Calculating every frame in this manner is wasteful
        let ghost = engine.block.ghost(&engine.field);

        for y in engine.field.hidden..engine.field.height {
            for x in 0..engine.field.width {
                renderer.set_draw_color(match (engine.field.occupies((x, y)), engine.block.occupies((x, y)), ghost.occupies((x, y))) {
                    (true, true,  _)      => Color::RGB(255, 0, 0),
                    (true, false, _)      => Color::RGB(150, 208, 246),
                    (false, true, _)      => Color::RGB(150, 108, 246),
                    (false, false, true)  => Color::RGB(120, 108, 146),
                    (false, false, false) => Color::RGB(0, 0, 0)
                });

                let _ = renderer.fill_rect(Rect::new(100 + 15 * x as i32,
                                                     10 + 15 * (y - engine.field.hidden) as i32, 15, 15));
            }
        }

        let xoffset = 115 + 15 * engine.field.width as u32;
        let mut yoffset = 25;

        renderer.set_draw_color(Color::RGB(150, 108, 246));

        // Draw preview pieces
        for id in engine.randomizer.preview(7) {
            for &(x, y) in engine.block.rs.data(id, Rotation::R0) {
                let _ = renderer.fill_rect(Rect::new((xoffset + 15 * x as u32) as i32, (yoffset + 15 * y as u32) as i32, 15, 15));
            }
            yoffset += 4 * 15 + 15;
        }

        renderer.set_draw_color(Color::RGB(255, 255, 255));
        let _ = renderer.draw_rect(Rect::new(100 - 1, 10 - 1, 15 * engine.field.width as u32 + 2,
                                             15 * (engine.field.height - engine.field.hidden) as u32 + 2));

        renderer.present();

        thread::sleep(Duration::from_millis(16));
    }
}

