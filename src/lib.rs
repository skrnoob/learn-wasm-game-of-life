mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;
extern crate js_sys;

extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

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
    cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.height - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8
            }
        }
        count
    }

    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    } 

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                log!(
                    "cell[{}, {}] is initially {:?} and has {} live neighbors",
                    row,
                    col,
                    cell,
                    live_neighbors
                );

                next.set(idx, match (cell, live_neighbors) {
                    (true, x) if x<2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise
                });
            }
        }

        self.cells = next;

    }

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;

        let size = (width*height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        
        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5);
            // cells.set(i, i % 2 == 0 || i % 7 == 0);
        }


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

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn set_width(&mut self, width:u32) {
        self.width = width;
        let size = (width * self.height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, false);
        }
        self.cells = cells;
    }

    pub fn set_height(&mut self, height:u32) {
        self.height = height;
        let size = (height * self.width) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, false);
        }

        self.cells = cells;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // for line in self.cells.as_slice().chunks(self.width as usize) {
        //     for &cell in line {
        //         let symbol = if cell == 0 { '◻' } else { '◼' };
        //         write!(f, "{}", symbol)?;

        //     }
        //     write!(f, "\n")?;
        // }

        let size = self.cells.len() as usize ;

        for idx in 0..size {
            let cell = self.cells.contains(idx);
            let symbol = if cell { '◼' } else { '◻' };
            write!(f, "{}", symbol)?;

            if (idx + 1) as u32 % self.width == 0 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}
