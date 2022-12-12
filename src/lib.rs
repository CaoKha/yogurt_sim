mod utils;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let row_above = (row + self.width - 1) % self.width;
        let row_below = (row + 1) % self.width;
        let column_left = (column + self.height - 1) % self.height;
        let column_right = (column + 1) % self.height;

        let mut count = 0;
        count += self.cells[self.get_index(row_above, column_left)] as u8;
        count += self.cells[self.get_index(row_above, column)] as u8;
        count += self.cells[self.get_index(row_above, column_right)] as u8;
        count += self.cells[self.get_index(row, column_left)] as u8;
        count += self.cells[self.get_index(row, column_right)] as u8;
        count += self.cells[self.get_index(row_below, column_left)] as u8;
        count += self.cells[self.get_index(row_below, column)] as u8;
        count += self.cells[self.get_index(row_below, column_right)] as u8;

        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }
        self.cells = next;
    }

    pub fn new(u_height: u32, u_width: u32) -> Universe {
        let width = u_height;
        let height = u_width;

        let size = (width * height) as usize;
        // init a cells
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, rand::random());
        }

        Universe {
            width,
            height,
            cells,
        }
    }
}
