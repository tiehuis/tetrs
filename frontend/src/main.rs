#![feature(collections, enumset)]
extern crate collections;
use collections::enum_set::CLike;

extern crate sdl2;
extern crate sdl2_ttf;
extern crate env_logger;
extern crate tetrs;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use tetrs::import::*;
use tetrs::controller::Action;

use std::thread;
use std::time::Duration;
use std::path::Path;

static COLORMAP: [Color; 7] = [
    Color::RGB(0, 255, 255), // I
    Color::RGB(128, 0, 128), // T
    Color::RGB(255, 165, 0), // L
    Color::RGB(0, 0, 255),   // J
    Color::RGB(128, 255, 0), // S
    Color::RGB(255, 0, 0),   // Z
    Color::RGB(255, 255, 0)  // O
];

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
    engine.co.deactivate_all();

    for &(scancode, action) in KEYMAP.iter() {
        if pump.keyboard_state().is_scancode_pressed(scancode) {
            engine.co.activate(action);
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

macro_rules! sq {
    ($x:expr, $y:expr, $s:expr) => {
        Rect::new($x as i32, $y as i32, $s as u32, $s as u32)
    }
}

macro_rules! render_text {
    ($renderer:expr, $font:expr; $msg:expr, $rect:expr) => {
        {
            let surface = $font.render($msg).blended(Color::RGBA(255, 255, 255, 255)).unwrap();
            let mut texture = $renderer.create_texture_from_surface(&surface).unwrap();
            $renderer.copy(&mut texture, None, Some($rect));
        }
    }
}

const LEFT_FIELD_POSITION: u32 = 130;

const UPPER_MARGIN: u32 = 60;

const UPPER_MARGIN2: u32 = 80;

fn main() {

    env_logger::init().unwrap();

    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let ttf_ctx = sdl2_ttf::init().unwrap();

    let window = match video_ctx.window("tetrs", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err)     => panic!("failed to create renderer: {}", err)
    };

    let mut events = ctx.event_pump().unwrap();

    let font = ttf_ctx.load_font(Path::new("res/font/font.ttf"), 128).unwrap();

    let options = EngineOptions::from_file("config.json");
    let mut engine = Engine::new(options);

    while engine.running {
        gather_input(&mut engine, &mut events);

        engine.update();

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        // Calculating every frame in this manner is wasteful
        let ghost = engine.bk.ghost(&engine.fd);

        for y in engine.fd.hidden..engine.fd.height {
            for x in 0..engine.fd.width {
                renderer.set_draw_color(match (engine.fd.occupies((x, y)), engine.bk.occupies((x, y)), ghost.occupies((x, y))) {
                    (true, true,  _)      => Color::RGB(255, 0, 0),
                    (true, false, _)      => COLORMAP[engine.fd.get((x, y)) as usize],
                    (false, true, _)      => COLORMAP[engine.bk.id as usize],
                    (false, false, true)  => {
                        let (r, g, b) = COLORMAP[engine.bk.id as usize].rgb();
                        Color::RGBA(20 + r / 7, 20 + g / 7, 20 + b / 7, 50)
                    },
                    (false, false, false) => Color::RGB(0, 0, 0)
                });

                let _ = renderer.fill_rect(sq!(LEFT_FIELD_POSITION + 15 * x as u32,
                                               UPPER_MARGIN + 15 * (y - engine.fd.hidden) as u32, 15));
            }
        }

        renderer.set_draw_color(Color::RGB(255, 255, 255));
        let _ = renderer.draw_rect(Rect::new(LEFT_FIELD_POSITION as i32 - 1, UPPER_MARGIN as i32 - 1, 15 * engine.fd.width as u32 + 2,
                                             15 * (engine.fd.height - engine.fd.hidden) as u32 + 2));


        let xoffset = LEFT_FIELD_POSITION + 20 + 15 * engine.fd.width as u32;
        let mut yoffset = UPPER_MARGIN2;

        // Draw preview pieces
        for id in engine.rd.preview(3) { //engine.op.preview_count as usize) {
            renderer.set_draw_color(COLORMAP[id as usize]);
            for &(x, y) in engine.bk.rs.data(id, Rotation::R0) {
                let _ = renderer.fill_rect(sq!(xoffset + 15 * x as u32, yoffset + 15 * y as u32, 15));
            }
            yoffset += 4 * 15 + 15;
        }

        // Draw hold piece
        if engine.hd.is_some() {
            renderer.set_draw_color(COLORMAP[engine.hd.unwrap() as usize]);
            for &(x, y) in engine.bk.rs.data(engine.hd.unwrap(), Rotation::R0) {
                let _ = renderer.fill_rect(sq!(LEFT_FIELD_POSITION - 15 * 4 - 20 + 15 * x as u32, UPPER_MARGIN2 + 15 * y as u32, 15));
            }
        }

        // Place text past the right previews
        let right_position = (xoffset + 15 * 5 + 40) as i32;
        let mut yoffset2 = (UPPER_MARGIN2 + 15) as i32;

        // Draw informational text
        render_text!(renderer, font; &format!("Lines Cleared: {}", engine.st.lines),
                     Rect::new(right_position, yoffset2, 150, 30));
        yoffset2 += 60;

        render_text!(renderer, font; &format!("Pieces: {}", engine.st.pieces),
                     Rect::new(right_position, yoffset2, 150, 30));
        yoffset2 += 60;

        render_text!(renderer, font; &format!("PPM: {:.5}", (engine.st.pieces as f64 /
                                                         (engine.tick_count * engine.mspt) as f64) * 1000_f64),
                     Rect::new(right_position, yoffset2, 150, 30));
        yoffset2 += 60;

        render_text!(renderer, font; &format!("Ticks: {}", engine.tick_count),
                     Rect::new(right_position, yoffset2, 150, 30));

        renderer.present();

        thread::sleep(Duration::from_millis(engine.mspt));
    }
}

