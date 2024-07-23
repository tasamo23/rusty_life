use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

use crate::Cell;

use super::Universe;

pub struct Renderer {
    context: CanvasRenderingContext2d,
    scale: u32,
}

impl Renderer {
    pub fn draw_cells(&self, universe: &Universe) {
        let context = &self.context;

        context.begin_path();
        context.clear_rect(
            0.0,
            0.0,
            (universe.width * self.scale) as f64,
            (universe.height * self.scale) as f64,
        );

        context.set_fill_style(&JsValue::from_str("black"));

        for x in 0..universe.width {
            for y in 0..universe.height {
                if universe.cell_at_index(x, y) == Cell::Dead {
                    context.fill_rect(
                        (x * self.scale) as f64,
                        (y * self.scale) as f64,
                        self.scale as f64,
                        self.scale as f64,
                    );
                }
            }
        }
    }

    pub fn new(canvas: HtmlCanvasElement, scale: u32) -> Renderer {
        let context = get_context_of(&canvas);

        Renderer { context, scale }
    }
}

pub fn create_canvas(width: u32, height: u32, scale: u32) -> HtmlCanvasElement {
    let window = window().expect("No global window found");

    let document = window
        .document()
        .expect("Could not find document in window");

    let body = document.body().expect("Could not find body in document");

    let canvas = document
        .create_element("canvas")
        .expect("Could not create HTML canvas element")
        .dyn_into::<HtmlCanvasElement>()
        .expect("Could not cast to HTML canvas element");

    body.append_child(&canvas)
        .expect("Could not append canvas to HTML body");

    canvas.set_width(width * scale);
    canvas.set_height(height * scale);

    canvas
}

fn get_context_of(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .expect("Could not get Canvas 2D context")
        .expect("Context is empty")
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("Could not cast to 2D context")
}
