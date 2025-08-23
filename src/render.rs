
use rand::SeedableRng;

use crate::transform::WorldToScreenTransform;
use crate::vmath::*;
use crate::render_target::*;
use crate::transform::{Transform, ModelTransform, CameraTransform};

pub struct Renderer {
    pub target: RenderTarget,
}

impl Renderer {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            target: RenderTarget::new(width, height),
        }
    }

    pub fn clear_color(self: &mut Self, color: Vec4) {
        self.target.color_buffer.iter_mut().for_each(|x| *x = color);
    }

    pub fn draw_triangles(&mut self, vert_buf: &Vec<Vertex>, index_buf: &Vec<u32>, in_model: ModelTransform, in_camera: CameraTransform, in_wts: WorldToScreenTransform) {
        if index_buf.len() == 0 {
            return;
        }

        let mut rng = rand::rngs::StdRng::from_seed([128;32]);

        let mut model = in_model.clone();
        let mut camera = in_camera.clone();
        let mut wts = in_wts.clone();

        model.calculate_transform();
        camera.calculate_transform();
        wts.calculate_transform();
        
        let triangle_count = index_buf.len() / 3;
        for i in 0..triangle_count {
            let face_idx = i*3;
            let i1 = index_buf[face_idx+0];
            let i2 = index_buf[face_idx+1];
            let i3 = index_buf[face_idx+2];

            let mut v1 = vert_buf[i1 as usize].clone();
            let mut v2 = vert_buf[i2 as usize].clone();
            let mut v3 = vert_buf[i3 as usize].clone();

            model.apply_transform(&mut v1.position);
            model.apply_transform(&mut v2.position);
            model.apply_transform(&mut v3.position);

            camera.apply_transform(&mut v1.position);
            camera.apply_transform(&mut v2.position);
            camera.apply_transform(&mut v3.position);

            wts.apply_transform(&mut v1.position);
            wts.apply_transform(&mut v2.position);
            wts.apply_transform(&mut v3.position);

            let p1 = IVec2::new(v1.position.x as i32, v1.position.y as i32);
            let p2 = IVec2::new(v2.position.x as i32, v2.position.y as i32);
            let p3 = IVec2::new(v3.position.x as i32, v3.position.y as i32);

            let bound_start_x = clamp(p1.x.min(p2.x.min(p3.x)), 0, self.target.width-1);
            let bound_start_y = clamp(p1.y.min(p2.y.min(p3.y)), 0, self.target.height-1);
            let bound_end_x = clamp(p1.x.max(p2.x.max(p3.x)), 0, self.target.width-1);
            let bound_end_y = clamp(p1.y.max(p2.y.max(p3.y)), 0, self.target.height-1);

            let p1f32 = p1.as_f32();
            let p2f32 = p2.as_f32();
            let p3f32 = p3.as_f32();

            let mut col = Vec4::rand_01(&mut rng);
            col.w = 1.0;

            for y in bound_start_y..=bound_end_y {
                for x in bound_start_x..=bound_end_x {
                    if is_point_in_triangle(Vec2::new(x as f32, y as f32), p1f32, p2f32, p3f32) {
                        self.target.color_buffer[(y * self.target.width + x) as usize] = col;
                    }
                }
            }
        }
    }
}

