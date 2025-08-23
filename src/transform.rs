
use crate::vmath::*;


pub trait Transform {
    fn calculate_transform(&mut self);

    fn apply_transform(&self, p: &mut Vec3);
}

#[derive(Clone)]
pub struct ModelTransform {
    pub translate: Vec3,
    pub yaw: f32,
    pub pitch: f32,

    ihat: Vec3,
    jhat: Vec3,
    khat: Vec3,
}

pub fn calculate_rotation(yaw: f32, pitch: f32, inverse: bool) -> (Vec3, Vec3, Vec3) {

    let yaw_ihat = Vec3::new(f32::cos(yaw), 0.0, f32::sin(yaw));
    let yaw_jhat = Vec3::new(0.0, 1.0, 0.0);
    let yaw_khat = Vec3::new(-f32::sin(yaw), 0.0, f32::cos(yaw));

    let pitch_ihat = Vec3::new(1.0, 0.0, 0.0);
    let pitch_khat = Vec3::new(0.0, f32::cos(pitch),-f32::sin(pitch));
    let pitch_jhat = Vec3::new(0.0, f32::sin(pitch), f32::cos(pitch));

    let ihat: Vec3;
    let jhat: Vec3;
    let khat: Vec3;
    
    if inverse {
        ihat = apply_rotation(pitch_ihat, pitch_jhat, pitch_khat, yaw_ihat);
        jhat = apply_rotation(pitch_ihat, pitch_jhat, pitch_khat, yaw_jhat);
        khat = apply_rotation(pitch_ihat, pitch_jhat, pitch_khat, yaw_khat);
    }
    else {
        ihat = apply_rotation(yaw_ihat, yaw_jhat, yaw_khat, pitch_ihat);
        jhat = apply_rotation(yaw_ihat, yaw_jhat, yaw_khat, pitch_jhat);
        khat = apply_rotation(yaw_ihat, yaw_jhat, yaw_khat, pitch_khat);
    }
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

    (ihat, jhat, khat)
}

pub fn apply_rotation(ihat: Vec3, jhat: Vec3, khat: Vec3, p: Vec3) -> Vec3 {
    ihat * p.x + jhat * p.y + khat * p.z
}

impl ModelTransform {
    pub fn new(translate: Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            translate: translate,
            yaw: yaw,
            pitch: pitch,

            ihat: Vec3::ZERO,
            jhat: Vec3::ZERO,
            khat: Vec3::ZERO,
        }
    }

}

impl Transform for ModelTransform {
    fn calculate_transform(&mut self) {
        (self.ihat, self.jhat, self.khat) = calculate_rotation(self.yaw, self.pitch, false);
    }

    fn apply_transform(&self, p: &mut Vec3) {
        *p = apply_rotation(self.ihat, self.jhat, self.khat, *p);

        p.x += self.translate.x;
        p.y += self.translate.y;
        p.z += self.translate.z;
    }
}

#[derive(Clone)]
pub struct CameraTransform {
    pub translate: Vec3,
    pub yaw: f32,
    pub pitch: f32,

    ihat: Vec3,
    jhat: Vec3,
    khat: Vec3,
}


impl CameraTransform {
    pub const fn new(translate: Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            translate: translate,
            yaw: yaw,
            pitch: pitch,

            ihat: Vec3::ZERO,
            jhat: Vec3::ZERO,
            khat: Vec3::ZERO,
        }
    }
}

impl Transform for CameraTransform {
    fn calculate_transform(&mut self) {
        (self.ihat, self.jhat, self.khat) = calculate_rotation(-self.yaw, -self.pitch, true);
    }

    fn apply_transform(&self, p: &mut Vec3) {
        //*p = apply_rotation(self.ihat, self.jhat, self.khat, *p);

        p.x -= self.translate.x;
        p.y -= self.translate.y;
        p.z -= self.translate.z;
    }
}

#[derive(Clone)]
pub struct WorldToScreenTransform {
    // given
    fov: f32,
    width: f32,
    height: f32,
    z_near: f32,
    z_far: f32,
    ar: f32,

    // computed
    tan_half_fov: f32,
}

impl WorldToScreenTransform {
    pub const fn new(fov_degrees: f32, width: f32, height: f32, z_near: f32, z_far: f32) -> Self {
        Self {
            fov: fov_degrees,
            width: width,
            height: height,
            z_near: z_near,
            z_far: z_far,
            tan_half_fov: 0.0,
            ar: width / height,
        }
    }
}

impl Transform for WorldToScreenTransform {
    fn calculate_transform(&mut self) {
        self.tan_half_fov = f32::tan((self.fov/2.0).to_radians());
    }

    fn apply_transform(&self, p: &mut Vec3) {
        assert!(self.tan_half_fov != 0.0);

        let f = (self.height / 2.0) / self.tan_half_fov;

        p.x = (p.x * f) / p.z;
        p.y = (p.y * f) / p.z;

        p.x += self.width / 2.0;
        p.y += self.height / 2.0;

        //let top = self.z_near * self.tan_half_fov;
        //let right = top * self.ar;

        //p.x = p.x / (p.z / self.z_near);
        //p.y = p.y / (p.z / self.z_near);
        //p.x = map(p.x, -right, right, 0.0, self.width);
        //p.y = map(p.y, -top, top, 0.0, self.height);
    }
}

