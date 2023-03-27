mod utils;
extern crate js_sys;
use wasm_bindgen::prelude::*; //interface with JavaScript
                              // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
                              // allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen]
#[repr(u8)] //represent each cell as a single byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
//define cell
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
//define universe
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

//translate the row and column into an index into the cells vector
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    //counting the neighbours that are alive
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

//public methods, exported to JavaScript
#[wasm_bindgen]
impl Universe {
    // Compute the next generation from the current one. Tick event is controlled by JavaScript
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours dies, as if caused
                    // by underpopulation
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbors lives on to the next
                    // generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live neighbours dies, as if by
                    // overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours becomes a live
                    // cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
    // Define a constructor that initializes the universe with an interesting pattern of live and
    // dead cells, as well as a render method.

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        // Initial Example from Tutorial.
        /*

                let cells = (0..width * height)
                    .map(|i| {
                        if i % 2 == 0 || i % 7 == 0 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    })
                    .collect();
        */

        // Single Spaceship
        /*
                let spaceship_0 = width * (height / 2 - 1) + (width / 2 - 1);
                let spaceship_1 = spaceship_0 + 2;
                let spaceship_2 = spaceship_0 + width;
                let spaceship_3 = spaceship_2 + 1;
                let spaceship_4 = spaceship_3 + 1;
                let spaceship_5 = spaceship_3 + width;

                let cells = (0..width * height)
                    .map(|i| {
                        if i == spaceship_0
                            || i == spaceship_1
                            || i == spaceship_2
                            || i == spaceship_3
                            || i == spaceship_4
                            || i == spaceship_5
                        {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    })
                    .collect();
        */
        // Random Initialization with js_sys Math.Random

        let cells = (0..width * height)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
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
    
    // For testing
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width *self.height).map)(|_i| Cell::Dead).collect();
    }
}
