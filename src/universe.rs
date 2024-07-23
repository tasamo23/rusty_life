use js_sys::Math::random;
use renderer::create_canvas;
use renderer::Renderer;

use crate::wasm_bindgen;

pub mod renderer;

use crate::Cell;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    renderer: Renderer,
}

impl Universe {
    pub fn cell_at_index(&self, x: u32, y: u32) -> Cell {
        self.cells[self.get_index(x, y)]
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn live_neighbor_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;

        for offset_x in [self.width - 1, 0, 1].iter().cloned() {
            for offset_y in [self.height - 1, 0, 1].iter().cloned() {
                if (offset_x != 0) || (offset_y != 0) {
                    let index_x = (offset_x + x) % self.width;
                    let index_y = (offset_y + y) % self.height;
                    count += self.cell_at_index(index_x, index_y) as u8;
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
                let cell = self.cell_at_index(x, y);

                let live_neighbors = self.live_neighbor_count(x, y);

                let next_cell_state = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next_cells[self.get_index(x, y)] = next_cell_state;
            }
        }

        self.cells = next_cells;
    }

    pub fn new(width: u32, height: u32) -> Universe {
        let cells = (0..width * height)
            .map(|_| {
                if random() > 0.7 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect::<Vec<Cell>>();

        let scale = 4;

        let canvas = create_canvas(width, height, scale);

        let renderer = Renderer::new(canvas, scale);

        Universe {
            width,
            height,
            cells,
            renderer,
        }
    }

    pub fn render(&self) {
        self.renderer.draw_cells(self);
    }
}
