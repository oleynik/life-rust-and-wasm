mod utils;

use std::fmt;
use std::fmt::Formatter;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut result = 0;
        for delta_row in [self.height - 1, 0, 1] {
            for delta_column in [self.width - 1, 0, 1] {
                if delta_row == 0 && delta_column == 0 {
                    continue;
                }
                let current_row = (row + delta_row) % self.height;
                let current_column = (column + delta_column) % self.width;
                let idx = self.get_index(current_row, current_column);
                result += self.cells[idx] as u8;
            }
        }
        result
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(w: u32, h: u32, aprob: f32) -> Self {
        let width = w;
        let height = h;

        let cells = (0..width * height)
            .map(|i| {
                if js_sys::Math::random() < aprob as f64 {
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for i in 0..self.height {
            for j in 0..self.width {
                let neighbors = self.live_neighbor_count(i, j);
                let idx = self.get_index(i, j);
                let current = self.cells[idx];
                next[idx] = match (current, neighbors) {
                    (Cell::Alive, 2 | 3) => Cell::Alive,
                    (Cell::Alive, _) => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (c, _) => c,
                }
            }
        }
        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
