mod timer_console;
mod utils;
use std::fmt;
use wasm_bindgen::prelude::*;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;
use timer_console::Timer;
extern crate js_sys;
extern crate web_sys;
use std::mem;

// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!($($t)*).into());
//     }
// }

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
    cells_bak: FixedBitSet,
    cells_position: Vec<u32>,
    cell_size: u32,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        // for delta_row in [self.height - 1, 0, 1].iter().cloned() {
        //     for delta_col in [self.height - 1, 0, 1].iter().cloned() {
        //         if delta_row == 0 && delta_col == 0 {
        //             continue;
        //         }
        //         let neighbor_row = (row + delta_row) % self.height;
        //         let neighbor_col = (column + delta_col) % self.width;
        //         let idx = self.get_index(neighbor_row, neighbor_col);
        //         count += self.cells[idx] as u8
        //     }
        // }
        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };
        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };
        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;
        let n = self.get_index(north, column);
        count += self.cells[n] as u8;
        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;
        let w = self.get_index(row, west);
        count += self.cells[w] as u8;
        let e = self.get_index(row, east);
        count += self.cells[e] as u8;
        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;
        let s = self.get_index(south, column);
        count += self.cells[s] as u8;
        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

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
        let _timer = Timer::new("Universe::tick");
        // let mut next = {
        //     let _timer = Timer::new("allocate next cells");
        //     self.cells.clone()
        // };

        {
            let _timer = Timer::new("new generation");
            let mut positions = vec![];
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);
                    let res = match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    };

                    self.cells_bak.set(idx, res);

                    if res {
                        let x = col * (self.cell_size + 1) + self.cell_size / 2 + 1;
                        let y = row * (self.cell_size + 1) + self.cell_size / 2 + 1;
                        positions.push(x);
                        positions.push(y);
                    }
                }
            }

            self.cells_position = positions;
        }
        {
            let _timer = Timer::new("swap cells");

            let cells = &mut self.cells;
            let cells_bak = &mut self.cells_bak;

            mem::swap(cells, cells_bak);
        }
    }

    pub fn new(width: u32, height: u32, cell_size: u32) -> Universe {
        utils::set_panic_hook();

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        let cells_bak = FixedBitSet::with_capacity(size);
        let mut cells_position = vec![];
        for i in 0..size {
            let state: bool = js_sys::Math::random() < 0.5;
            cells.set(i, state);
            if state {
                let x = (i as u32 % width) * (cell_size + 1) + cell_size / 2 + 1;
                let y = js_sys::Math::floor(i as f64 / width as f64) as u32 * (cell_size + 1)
                    + cell_size / 2
                    + 1;
                cells_position.push(x);
                cells_position.push(y);
            }
        }

        Universe {
            width,
            height,
            cells,
            cells_bak,
            cells_position,
            cell_size,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cell_size(&self) -> u32 {
        self.cell_size
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn cells_position(&self) -> *const u32 {
        self.cells_position.as_slice().as_ptr()
    }

    pub fn cells_position_len(&self) -> u32 {
        self.cells_position.len() as u32
    }

    pub fn cells_to_cells_positon(&mut self) {
        let mut positions = vec![];
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                if cell {
                    let x = col * (self.cell_size + 1) + self.cell_size / 2 + 1;
                    let y = row * (self.cell_size + 1) + self.cell_size / 2 + 1;
                    positions.push(x);
                    positions.push(y);
                }
            }
        }

        self.cells_position = positions;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (width * self.height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, false);
        }
        self.cells = cells;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (height * self.width) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, false);
        }

        self.cells = cells;
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        match self.cells.contains(idx) {
            true => self.cells.set(idx, false),
            false => self.cells.set(idx, true),
        }
    }

    pub fn reset(&mut self, width: u32, height: u32, cells_size: u32) {
        *self = Universe::new(width, height, cells_size);
    }

    pub fn clear(&mut self) {
        let size = self.cells.len();
        for i in 0..size {
            self.cells.set(i, false);
        }
    }

    pub fn set_glider(&mut self, row: u32, column: u32) {
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);

                match (delta_row, delta_col) {
                    (0, 1) | (1, _) => {
                        self.cells.set(idx, true);
                    }
                    (height, 0) if height == self.height - 1 => {
                        self.cells.set(idx, true);
                    }
                    (_, _) => {
                        self.cells.set(idx, false);
                    }
                }
            }
        }
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

        let size = self.cells.len() as usize;

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
