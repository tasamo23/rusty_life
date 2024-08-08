use js_sys::Math::random;
use renderer::create_canvas;
use renderer::Renderer;

use crate::wasm_bindgen;

extern crate fixedbitset;

use fixedbitset::FixedBitSet;

pub mod renderer;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
    renderer: Renderer,
    mutation_rate: f64,
    dynamic_mutation_rate: bool,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let mut cells = FixedBitSet::with_capacity((width * height) as usize);

        for i in 0..(width * height) {
            cells.set(i as usize, random() > 0.7);
        }

        let scale = 10;

        let mutation_rate = 0.01;
        let dynamic_mutation_rate = false;

        let canvas = create_canvas(width, height, scale);

        let renderer = Renderer::new(canvas, scale);

        Universe {
            width,
            height,
            cells,
            renderer,
            mutation_rate,
            dynamic_mutation_rate,
        }
    }

    pub fn render(&self) {
        self.renderer.draw_cells(self);
    }

    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.cell_at_index(x, y);

                let live_neighbors = self.live_neighbor_count(x, y);

                let next_cell_state = match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (false, x) if x > 0 => random() < self.mutation_rate,
                    (otherwise, _) => otherwise,
                };

                next_cells.set(self.get_index(x, y), next_cell_state);
            }
        }

        if self.dynamic_mutation_rate {
            self.mutation_rate = (self.cells.count_zeroes(..) / (self.width * self.height) as usize)
                .pow(4) as f64
                * 0.01;
        }

        self.cells = next_cells;
    }

    pub fn set_mutation_rate(&mut self, new_rate: f64) {
        self.mutation_rate = new_rate
    }

    pub fn set_dynamic_mutation_rate(&mut self, new_val: bool) {
        self.dynamic_mutation_rate = new_val
    }
}

impl Universe {
    pub fn cell_at_index(&self, x: u32, y: u32) -> bool {
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
                    count += if self.cell_at_index(index_x, index_y) {
                        1
                    } else {
                        0
                    };
                }
            }
        }
        count
    }
}
