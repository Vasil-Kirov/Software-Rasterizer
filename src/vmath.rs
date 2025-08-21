
use std::{fmt::Formatter, ops::*};
use rand::Rng;

pub fn clamp<T: std::cmp::PartialOrd>(x: T, from: T, to: T) -> T {
    if x < from { from } else if x > to { to } else { x }
}

#[allow(dead_code)]
pub fn lerp(a: f32, b:f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn map<T>(x: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy
{
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn world_to_screen(world: Vec4, w: i32, h: i32) -> IVec2 {
    IVec2::new(
        map(world.x, 0.0, 5.0, 0.0, w as f32) as i32,
        map(world.y, 0.0, 2.8125, 0.0, h as f32) as i32,
    )
}

pub fn dot(a: Vec2, b: Vec2) -> f32 {
    a.x * b.x + a.y * b.y
}

pub fn is_point_on_rightside_of_line(point: Vec2, a: Vec2, b: Vec2) -> bool {
    let ab = b - a;
    let rotated = Vec2::new(ab.y, -ab.x);

    let ap = point - a;

    dot(ap, rotated) <= 0.0
}

pub fn is_point_in_triangle(point: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let check0 = is_point_on_rightside_of_line(point, a, b);
    let check1 = is_point_on_rightside_of_line(point, b, c);
    let check2 = is_point_on_rightside_of_line(point, c, a);
    return check0 && check1 && check2;
}

#[derive(Clone, PartialEq)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2
}

impl Vertex {
    #[inline]
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
        Self { position: position, normal: normal, tex_coords: tex_coords }
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
impl IVec2 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    /// All ones.
    pub const ONE: Self = Self::splat(1);

    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    pub const fn splat(v: i32) -> Self {
        Self { x: v, y: v }
    }

    pub const fn as_f32(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}


#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vec2 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
    }

    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Index<usize> for Vec2 {
    type Output = f32;

    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of range!"),
        }
    }
}

impl IndexMut<usize> for Vec2 {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of range!"),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl Vec3 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }

    pub const fn xy(self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }

    pub fn len(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.len()
    }

    pub fn v4(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, 1.0)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }

}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range!"),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[allow(dead_code)]
impl Vec4 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    #[inline]
    pub fn rand_01<T: Rng>(rng: &mut T) -> Self {
        Self {
            x: rng.gen_range(0.0..1.0),
            y: rng.gen_range(0.0..1.0),
            z: rng.gen_range(0.0..1.0),
            w: rng.gen_range(0.0..1.0),
        }
    }

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x: x, y: y, z: z, w: w }
    }

    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v, z: v, w: v }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

