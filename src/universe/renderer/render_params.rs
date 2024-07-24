use wasm_bindgen::JsValue;

pub struct RenderParams {
    pub cell_alive_color: Color,
    pub cell_dead_color: Color,
    pub universe_color: Color,
}

impl RenderParams {
    pub fn default() -> RenderParams {
        RenderParams {
            cell_alive_color: Color::rgb(255, 255, 255),
            cell_dead_color: Color::rgb(0, 0, 0),
            universe_color: Color::rgb(100, 100, 100),
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

    pub fn to_JsValue(&self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::RGB { r, g, b }
    }

    pub fn oklch(l: f64, c: f64, h: f64) -> Color {
        Color::OKLCH { l, c, h }
    }
}
