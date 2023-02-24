use crate::engine::cell::{Cell, Type};
use std::collections::HashMap;
use std::fmt::Display;

pub struct Game {
    pub board: Board,
    pub type_table: HashMap<Type, HashMap<Type, f32>>,
    pub renderer: crate::engine::render::Renderer,
}

impl Game {
    pub fn new((width, height): (usize, usize), (w_win, h_win): (u32, u32)) -> Self {
        let mut cells: Vec<Cell> = Vec::new();
        for _ in 0..width * height {
            cells.push(Cell::new(
                Type::TYPES[rand::random::<usize>() % Type::TYPES.len()],
            ));
        }
        Self {
            board: Board {
                cells,
                width,
                height,
            },
            type_table: Type::type_table(),
            renderer: crate::engine::render::Renderer::setup((w_win, h_win)),
        }
    }

    /// A tick is when the engine updates:
    /// - The cells, who wins and who loses
    /// - The input (optional)
    /// # Parameters
    /// - Tick speed: How quick the engine updates
    fn tick(&mut self) {
        for y in 0..self.board.height {
            for x in 0..self.board.width {
                let mut neighbors = self.board.get_neighbors(x as i32, y as i32);
                for neighbor in neighbors.iter_mut() {
                    self.attack_cell((x, y), (neighbor.0, neighbor.1));
                }
            }
        }
    }

    /// Goes on until the program is exited.
    pub fn game_loop(&mut self, tick_speed: f64) {
        for cell in self.board.cells.iter_mut() {
            cell.changed = false;
        }

        'game_loop: loop {
            // let now = std::time::Instant::now();
            self.tick();
            // println!("Tick time: {:?}ms", now.elapsed().as_millis());
            // let now = std::time::Instant::now();
            if crate::engine::render::Renderer::render_optimized(self) {
                // Renders return true if requested to quit
                break 'game_loop;
            }
            // println!("Draw time: {:?}ms", now.elapsed().as_millis());
            std::thread::sleep(std::time::Duration::from_millis(
                (1000.0 / tick_speed) as u64,
            ));

            for cell in self.board.cells.iter_mut() {
                cell.changed = false;
            }
        }
    }

    fn attack_cell(&mut self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) {
        if self.board.cells[x2 + y2 * self.board.width].cell_type
            == self.board.cells[x1 + y1 * self.board.width].cell_type
            || self.board.cells[x1 + y1 * self.board.width].changed
            || self.board.cells[x2 + y2 * self.board.width].changed
        {
            return;
        }

        let damage = *Cell::get_damage(
            self,
            self.board.cells[x1 + y1 * self.board.width],
            self.board.cells[x2 + y2 * self.board.width],
        );

        self.board.cells[x2 + y2 * self.board.width].health -= damage;
        if self.board.cells[x2 + y2 * self.board.width].health <= 0.0 {
            self.board.cells[x2 + y2 * self.board.width] =
                Cell::new(self.board.cells[x1 + y1 * self.board.width].cell_type);
        } else {
            self.board.cells[x2 + y2 * self.board.width].changed = false;
        }
    }
}

#[derive(Clone)]
pub struct Board {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    /// Get all 8 neighbors of a cell
    ///
    /// Loops back on the sides and on the top, like a sphere
    pub fn get_neighbors(&self, x: i32, y: i32) -> Vec<(usize, usize)> {
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        for y_offset in -1..=1 {
            for x_offset in -1..=1 {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }
                let x = (x + x_offset + self.width as i32) % self.width as i32;
                let y = (y + y_offset + self.height as i32) % self.height as i32;
                neighbors.push((x as usize, y as usize));
            }
        }
        neighbors
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cells[x + y * self.width];
                write!(f, " {} : {:?} |", cell.health, cell.cell_type)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
