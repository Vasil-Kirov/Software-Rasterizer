

use crate::vmath::*;

pub struct ModelTransform {
    pub translate: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}


impl ModelTransform {
    pub fn new(translate: Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            translate: translate,
            yaw: yaw,
            pitch: pitch,
        }
    }

    pub fn apply_rotation(ihat: Vec3, jhat: Vec3, khat: Vec3, p: Vec3) -> Vec3 {
        ihat * p.x + jhat * p.y + khat * p.z
    }

    pub fn apply_transform(&self, p: &mut Vec3) {

        let yaw_ihat = Vec3::new(f32::cos(self.yaw), 0.0, f32::sin(self.yaw));
        let yaw_jhat = Vec3::new(0.0, 1.0, 0.0);
        let yaw_khat = Vec3::new(-f32::sin(self.yaw), 0.0, f32::cos(self.yaw));

        let pitch_ihat = Vec3::new(1.0, 0.0, 0.0);
        let pitch_khat = Vec3::new(0.0, f32::cos(self.pitch),-f32::sin(self.pitch));
        let pitch_jhat = Vec3::new(0.0, f32::sin(self.pitch), f32::cos(self.pitch));

        let ihat = Self::apply_rotation(yaw_ihat, yaw_jhat, yaw_khat, pitch_ihat);
        let jhat = Self::apply_rotation(yaw_ihat, yaw_jhat, yaw_khat, pitch_jhat);
        let khat = Self::apply_rotation(yaw_ihat, yaw_jhat, yaw_khat, pitch_khat);

        *p = Self::apply_rotation(ihat, jhat, khat, *p);

        p.x += self.translate.x;
        p.y += self.translate.y;
        p.z += self.translate.z;

        /*
         * rotate around y
         * i^ = cos(a), 0,      sin(a)
         * j^ = 0,      1,      0    
         * k^ = -sin(a),0,      cos(a)
         *
         * rotate around x
         * i^ = 1,      0,     0
         * j^ = 0,      cos(a),-sin(a)
         * k^ = 0,      sin(a),cos(a)
         *
         * y axis rotation
         *
         *  pitch = x
         *  yaw = y
         *
         */

    }
}

