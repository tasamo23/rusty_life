use std::f64::consts::PI;

use js_sys::Math::{cos, sin};
use wasm_bindgen::JsValue;

pub struct RenderParams {
    pub cell_alive_color: Color,
    pub cell_dead_color: Color,
    pub universe_color: Color,
    pub force_rgb: bool,
}

impl RenderParams {
    pub fn default() -> RenderParams {
        RenderParams {
            cell_alive_color: Color::rgb(255, 255, 255),
            cell_dead_color: Color::rgb(0, 0, 0),
            universe_color: Color::rgb(100, 100, 100),
            force_rgb: false,
        }
    }
}

pub enum Color {
    RGB { r: u8, g: u8, b: u8 },
    OKLCH { l: f64, c: f64, h: f64 },
}

impl Color {
    fn to_string(&self) -> String {
        match self {
            Color::RGB { r, g, b } => {
                format!("rgb({r},{g},{b})")
            }
            Color::OKLCH { l, c, h } => {
                format!("oklch({l},{c},{h})")
            }
        }
    }

    pub fn to_js_value(&self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::RGB { r, g, b }
    }

    pub fn oklch(l: f64, c: f64, h: f64) -> Color {
        Color::OKLCH { l, c, h }
    }

    pub fn to_rgb(color: Color) -> Color {
        // https://gist.github.com/dkaraush/65d19d61396f5f3cd8ba7d1b4b3c9432
        match color {
            Color::RGB { r, g, b } => Color::rgb(r, g, b),
            Color::OKLCH { l, c, h } => {
                let color_vals = [l, c, h];

                let oklab = [l, c * cos(h * PI / 180.0), c * sin(h * PI / 180.0)];

                Color::OKLCH { l, c, h }
            }
        }
    }
}
