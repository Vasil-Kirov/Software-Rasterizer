
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

use self::transform::CameraTransform;
use self::transform::WorldToScreenTransform;


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
    
    let mut camera_pos = Vec3::ZERO;

    rl.set_target_fps(144);
    while !rl.window_should_close() {
        renderer.clear_color(Vec4::new(0.258824, 0.258824, 0.435294, 1.0f32));

        let model = ModelTransform::new(Vec3::new(0.0, 0.0, 4.0), yaw, 0.0);
        yaw += 0.2 * delta_t as f32;

        let camera = CameraTransform::new(camera_pos, 0.0, 0.0);
        let persp = WorldToScreenTransform::new(120.0, 1280.0, 720.0, 0.1, 100.0);

        renderer.draw_triangles(&cube.verts, &cube.indices, model, camera, persp);

        let pixels = renderer.target.color_buffer_to_pixels();

        {
            let mut d = rl.begin_drawing(&thread);

            _ = backbuffer_texture.update_texture(pixels.as_slice());

            d.draw_texture(&backbuffer_texture, 0, 0, color::rcolor(0xFF, 0xFF, 0xFF, 0xFF));
        }

        if rl.is_key_down(KeyboardKey::KEY_W)
        {
            camera_pos.z += delta_t;
        }
        if rl.is_key_down(KeyboardKey::KEY_S)
        {
            camera_pos.z -= delta_t;
        }
        if rl.is_key_down(KeyboardKey::KEY_D)
        {
            camera_pos.x += delta_t;
        }
        if rl.is_key_down(KeyboardKey::KEY_A)
        {
            camera_pos.x -= delta_t;
        }

        let end_frame_time = rl.get_time() / 1000.0;
        let ms_elapsed = end_frame_time - last_frame_time;

        delta_t = clamp(ms_elapsed / 1000.0, 0.0167, 0.1) as f32;

        last_frame_time = end_frame_time;
    }
}
