use crate::vmath::*;


pub struct RenderTarget {
    pub width: i32,
    pub height: i32,
    pub color_buffer: Box<[Vec4]>,
    pub depth_buffer: Box<[f32]>,
}

impl RenderTarget {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width: width,
            height: height,
            color_buffer: vec![Vec4::ZERO; (width * height) as usize].into_boxed_slice(),
            depth_buffer: vec![0.0; (width * height) as usize].into_boxed_slice(),
        }
    }

    pub fn color_buffer_to_pixels(self: &Self) -> Vec<u8> {
        let size = self.color_buffer.len();
        let mut res = vec![0u8; size*4];

        for y in 0..self.height {
            for x in 0..self.width {
                let v = self.color_buffer[(y * self.width + x) as usize];
                let scaled = v * 255.0;
                let at = ((self.height - y - 1) * (self.width * 4) + x * 4) as usize;

                res[at+0] = scaled.x as u8;
                res[at+1] = scaled.y as u8;
                res[at+2] = scaled.z as u8;
                res[at+3] = scaled.w as u8;
            }
        }

        res
    }

}

