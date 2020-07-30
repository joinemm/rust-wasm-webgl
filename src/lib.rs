mod programs;
mod shaders;
mod utils;
mod webgl_setup;

use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Client {
    gl: GL,
    program_color_2d: programs::color_2d::Color2D,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let gl = webgl_setup::initialize_context().unwrap();
        log("created new client");
        Self {
            program_color_2d: programs::color_2d::Color2D::new(&gl),
            gl: gl,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        self.program_color_2d
            .render(&self.gl, 0., 10., 0., 10., 10., 10.)
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    log("Hello from rust!");
    utils::set_panic_hook();
    Ok(())
}
