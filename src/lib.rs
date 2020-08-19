extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod constants;
mod programs;
mod shaders;
mod utils;
mod webgl_setup;

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
    program_color_2d_gradient: programs::color_2d_gradient::Color2DGradient,
    program_graph_3d: programs::graph_3d::Graph3D,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook();
        let gl = webgl_setup::initialize_context().unwrap();
        Self {
            program_color_2d: programs::color_2d::Color2D::new(&gl),
            program_color_2d_gradient: programs::color_2d_gradient::Color2DGradient::new(&gl),
            program_graph_3d: programs::graph_3d::Graph3D::new(&gl),
            gl: gl,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        app_state::update_dynamic_data(time, height, width);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        let curr_state = app_state::get_current_state();

        self.program_color_2d.render(
            &self.gl,
            curr_state.control_bottom,
            curr_state.control_top,
            curr_state.control_left,
            curr_state.control_right,
            curr_state.canvas_height,
            curr_state.canvas_width,
        );

        self.program_color_2d_gradient.render(
            &self.gl,
            curr_state.control_top - 200.,
            curr_state.control_top - 10.,
            curr_state.control_left + 10.,
            curr_state.control_left + 200.,
            curr_state.canvas_height,
            curr_state.canvas_width,
        );

        self.program_graph_3d.render(
            &self.gl,
            curr_state.control_bottom,
            curr_state.control_top,
            curr_state.control_left,
            curr_state.control_right,
            curr_state.canvas_height,
            curr_state.canvas_width,
            curr_state.rotation_x_axis,
            curr_state.rotation_y_axis,
            &utils::get_updated_3d_y_values(curr_state.time),
        );
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    utils::set_panic_hook();
    Ok(())
}
