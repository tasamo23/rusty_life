use render_params::RenderParams;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

pub mod render_params;

use super::Universe;

pub struct Renderer {
    context: CanvasRenderingContext2d,
    scale: u32,
    renderparams: RenderParams,
}

impl Renderer {
    pub fn new(canvas: HtmlCanvasElement, scale: u32) -> Renderer {
        let context = get_context_of(&canvas);

        let renderparams = RenderParams::default();

        Renderer {
            context,
            scale,
            renderparams,
        }
    }

    pub fn draw_cells(&self, universe: &Universe) {
        let context = &self.context;

        context.set_fill_style(&self.renderparams.universe_color.to_js_value());
        context.fill_rect(
            0.0,
            0.0,
            (universe.width * self.scale) as f64,
            (universe.height * self.scale) as f64,
        );

        context.begin_path();

        for x in 0..universe.width {
            for y in 0..universe.height {
                if universe.cell_at_index(x, y) {
                    context.set_fill_style(&self.renderparams.cell_alive_color.to_js_value());
                } else {
                    context.set_fill_style(&self.renderparams.cell_dead_color.to_js_value());
                }
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
