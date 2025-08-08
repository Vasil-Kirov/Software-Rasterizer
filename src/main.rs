
mod render;
mod vmath;
mod model;
mod render_target;

use raylib::prelude::*;
use raylib::color;
use render::Renderer;
use vmath::*;
use model::Model;


fn main() {
    const WIDTH: i32 = 1280;
    const HEIGHT: i32 = 720;
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Software Rasterizer").build();

    let mut backbuffer_texture = rl.load_render_texture(&thread, WIDTH as u32, HEIGHT as u32).unwrap();
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    let cube_file = include_str!("../cube.obj");
    let cube = Model::load_from_data(cube_file).unwrap();

    rl.set_target_fps(144);
    while !rl.window_should_close() {
        renderer.clear_color(Vec4::new(0.258824, 0.258824, 0.435294, 1.0f32));
        renderer.draw_triangles(&cube.verts, &cube.indices);
        //renderer.draw_triangle([Vec2::new(-0.5, -0.5), Vec2::new(0.5, -0.5), Vec2::new(0.0, 0.5)]);

        let pixels = renderer.target.color_buffer_to_pixels();
        let mut d = rl.begin_drawing(&thread);

        _ = backbuffer_texture.update_texture(pixels.as_slice());

        d.draw_texture(&backbuffer_texture, 0, 0, color::rcolor(0xFF, 0xFF, 0xFF, 0xFF));

    }
}
