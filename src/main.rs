
mod render;
mod vmath;
mod model;
mod render_target;
mod transform;

use raylib::prelude::*;
use raylib::color;
use render::Renderer;
use vmath::*;
use transform::{ModelTransform};
use model::Model;
use std::f32::consts::{PI};


fn main() {
    const WIDTH: i32 = 1280;
    const HEIGHT: i32 = 720;
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Software Rasterizer").build();

    rl.set_trace_log(TraceLogLevel::LOG_WARNING);

    let mut backbuffer_texture = rl.load_render_texture(&thread, WIDTH as u32, HEIGHT as u32).unwrap();
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    let cube_file = include_str!("../cube.obj");
    let cube = Model::load_from_data(cube_file).unwrap();
    let mut last_frame_time = rl.get_time() / 1000.0;
    let mut delta_t = 0.1;
    let mut yaw = PI/4.0;

    rl.set_target_fps(144);
    while !rl.window_should_close() {
        renderer.clear_color(Vec4::new(0.258824, 0.258824, 0.435294, 1.0f32));

        let model = ModelTransform::new(Vec3::new(2.0, 1.5, 0.0), yaw, 1.0);
        yaw += delta_t as f32;

        renderer.draw_triangles(&cube.verts, &cube.indices, model);

        let pixels = renderer.target.color_buffer_to_pixels();

        {
            let mut d = rl.begin_drawing(&thread);

            _ = backbuffer_texture.update_texture(pixels.as_slice());

            d.draw_texture(&backbuffer_texture, 0, 0, color::rcolor(0xFF, 0xFF, 0xFF, 0xFF));
        }

        let end_frame_time = rl.get_time() / 1000.0;
        let ms_elapsed = end_frame_time - last_frame_time;

        delta_t = clamp(ms_elapsed / 1000.0, 0.0167, 0.1);

        last_frame_time = end_frame_time;
    }
}
