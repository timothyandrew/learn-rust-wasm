mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    console::log_1(&"Hello!".into());
    console::warn_1(&"Warning!".into());
}
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn get_idx(&self, row: usize, column: usize) -> usize {
        (self.width * row) + column
    }

    fn live_neighbor_count(&self, row: usize, column: usize) -> u8 {
        [(0, 1), (0, -1), (-1, 0), (1, 0), (1, 1), (1, -1), (-1, -1), (-1, 1)]
            .iter()
            .fold(0, |acc, (row_delta, column_delta)| {
                let row = row_delta + (row + self.height) as isize;
                let column = column_delta + (column + self.width) as isize;

                let row = (row as usize) % self.height;
                let column = (column as usize) % self.width;

                let idx = self.get_idx(row, column);
                acc + self.cells[idx] as u8
            })
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_idx(row, column);
                let live_neighbors = self.live_neighbor_count(row, column);

                match (self.cells[idx], live_neighbors) {
                    (Cell::Alive, 2..=3) => (),
                    (Cell::Alive, _) => next[idx] = Cell::Dead,
                    (Cell::Dead, 3) => next[idx] = Cell::Alive,
                    (Cell::Dead, _) => ()
                };
            }
        }

        self.cells = next;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_idx(row, column);

                let c = match self.cells[idx] {
                    Cell::Alive => "◼",
                    Cell::Dead => "◻"
                };

                write!(f, "{}", c)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}