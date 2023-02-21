use crate::engine::{cell::Type, game::Game};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::WindowCanvas, EventPump};

pub struct Renderer {
    canvas: WindowCanvas,
    pub events: EventPump,
}

impl Renderer {
    pub fn setup((w, h): (u32, u32)) -> Renderer {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Pokemon's Game of Life", w, h);

        // If something panics, it's most likely there
        let mut canvas = window.build().unwrap().into_canvas().build().unwrap();
        canvas.set_draw_color(Color::BLACK);

        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            canvas,
            events: event_pump,
        }
    }

    // Only renders the changed cells
    pub fn render_optimized(game: &mut Game) -> bool {
        for event in game.renderer.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return true;
                }
                _ => {}
            };
        }

        game.renderer.canvas.present();
        for x in 0..game.board.width {
            for y in 0..game.board.height {
                let cell = &game.board.cells[x + y * game.board.width];
                if cell.changed {
                    Self::draw_cell(*cell, game, (x, y));
                }
            }
        }
        game.renderer.canvas.present();
        false
    }

    fn draw_cell(cell: crate::engine::cell::Cell, game: &mut Game, (x, y): (usize, usize)) {
        game.renderer
            .canvas
            .set_draw_color(get_color(cell.cell_type));
        game.renderer
            .canvas
            .fill_rect(sdl2::rect::Rect::new(
                (x * game.renderer.canvas.output_size().unwrap().0 as usize / game.board.width)
                    as i32,
                (y * game.renderer.canvas.output_size().unwrap().1 as usize / game.board.height)
                    as i32,
                (game.renderer.canvas.output_size().unwrap().0 as usize / game.board.width) as u32,
                (game.renderer.canvas.output_size().unwrap().1 as usize / game.board.height) as u32,
            ))
            .unwrap();
    }
}

fn get_color(t: Type) -> Color {
    match t {
        Type::Normal => Color::RGB(168, 167, 122),
        Type::Fire => Color::RGB(238, 129, 48),
        Type::Water => Color::RGB(99, 144, 240),
        Type::Grass => Color::RGB(122, 199, 76),
        Type::Electric => Color::RGB(247, 208, 44),
        Type::Ice => Color::RGB(150, 217, 214),
        Type::Fighting => Color::RGB(194, 46, 40),
        Type::Poison => Color::RGB(163, 62, 161),
        Type::Ground => Color::RGB(226, 191, 101),
        Type::Flying => Color::RGB(169, 143, 243),
        Type::Psychic => Color::RGB(249, 85, 135),
        Type::Bug => Color::RGB(166, 185, 26),
        Type::Rock => Color::RGB(182, 161, 54),
        Type::Ghost => Color::RGB(115, 87, 151),
        Type::Dragon => Color::RGB(111, 53, 252),
        Type::Dark => Color::RGB(112, 87, 70),
        Type::Steel => Color::RGB(183, 183, 206),
        Type::Fairy => Color::RGB(214, 133, 173),
    }
}
