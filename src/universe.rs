use crate::wasm_bindgen;

use crate::Cell;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn live_neighbor_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;

        for offset_x in [self.width - 1, 0, 1].iter().cloned() {
            for offset_y in [self.height - 1, 0, 1].iter().cloned() {
                let index_x = (offset_x + x) % self.width;
                let index_y = (offset_y + y) % self.height;

                if index_x != 0 && index_y != 0 {
                    count += self.cells[self.get_index(index_x, index_y)] as u8;
                }
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let index = self.get_index(x, y);
                let cell = self.cells[index];

                let live_neighbors = self.live_neighbor_count(x, y);

                let next_cell_state = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (otherwise, _) => otherwise,
                };

                next_cells[index] = next_cell_state;
            }
        }

        self.cells = next_cells;
    }

    pub fn new(width: u32, height: u32) -> Universe {
        let cells = (0..width * height)
            .map(|i| {
                if i % 7 == 0 || i % 3 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect::<Vec<Cell>>();
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) {}

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}
