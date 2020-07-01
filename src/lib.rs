mod utils;

use wasm_bindgen::prelude::*;
use rand::Rng;
use web_sys::console;
use fixedbitset::FixedBitSet;

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
pub struct Universe {
    width: usize,
    height: usize,
    cells: FixedBitSet
}

#[wasm_bindgen]
impl Universe {
    fn gen_cells<F: FnMut(usize) -> bool>(capacity: usize, mut pred: F) -> FixedBitSet {
        let mut bitset = FixedBitSet::with_capacity(capacity);

        for i in 0..capacity {
            if pred(i) { 
                bitset.set(i, true);
            } 
        }

        bitset
    }

    pub fn rand(width: usize, height: usize) -> Universe {
        let mut rng = rand::thread_rng();
        let cells = Universe::gen_cells(width * height, |_| rng.gen_bool(0.10));
        Universe { width, height, cells }
    }

    pub fn new() -> Universe {
        let width = 160;
        let height = 100;
        let cells = Universe::gen_cells(width * height, |i| i % 2 == 0 || i % 7 == 0);
        Universe { width, height, cells }
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
        self.cells = FixedBitSet::with_capacity(self.height * width);
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
        self.cells = FixedBitSet::with_capacity(self.width * height);
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
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
                let is_cell_alive = self.cells[idx];

                match (is_cell_alive, live_neighbors) {
                    (true, 2..=3) => (),
                    (true, _) => next.set(idx, false),
                    (false, 3) => next.set(idx, true),
                    (false, _) => ()
                };
            }
        }

        self.cells = next;
    }
}

impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(usize, usize)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_idx(row, col);
            self.cells.set(idx, true);
        }
    }
}