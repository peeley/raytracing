use std::ops;
use std::fmt;

#[derive(Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

pub type Coordinate = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn length_squared(&self) -> f32 {
        return self.x*self.x + self.y*self.y + self.z*self.z;
    }
    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point: (x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, vec: Vec3){
        self.x *= vec.x;
        self.y *= vec.y;
        self.z *= vec.z;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, x: f32){
        self.x *= x;
        self.y *= x;
        self.z *= x;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, x: f32){
        self.x /= x;
        self.y /= x;
        self.z /= x;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3{
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}
