use rand::{thread_rng, Rng};
use std::fmt;
use std::ops;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Coordinate = Vec3;
pub type Color = Vec3;

impl Color {
    pub fn print(&self, samples: i32) {
        let scale = 1.0 / samples as f32;
        println!(
            "{} {} {}",
            (255.999 * (self.x * scale).sqrt().clamp(0.0, 0.999)) as u8,
            (255.999 * (self.y * scale).sqrt().clamp(0.0, 0.999)) as u8,
            (255.999 * (self.z * scale).sqrt().clamp(0.0, 0.999)) as u8
        );
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
    pub fn length_squared(&self) -> f32 {
        return (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
    }
    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }
    pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
        return (u.x * v.x) + (u.y * v.y) + (u.z * v.z);
    }
    pub fn cross(u: &Vec3, v: &Vec3) -> Self {
        return Vec3 {
            x: (u.y * v.z) - (u.z * v.y),
            y: (u.z * v.x) - (u.x * v.z),
            z: (u.x * v.y) - (u.y * v.x),
        };
    }
    pub fn unit_vec(&self) -> Self {
        return (*self) / self.length();
    }
    pub fn random(min: Option<f32>, max: Option<f32>) -> Self {
        let mut rng = thread_rng();
        let min_val = min.unwrap_or(0.0);
        let max_val = max.unwrap_or(1.0);
        Vec3 {
            x: rng.gen_range(min_val, max_val),
            y: rng.gen_range(min_val, max_val),
            z: rng.gen_range(min_val, max_val),
        }
    }
    pub fn random_unit() -> Self {
        let mut rng = thread_rng();
        let a = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
        let z: f32 = rng.gen_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Vec3 {
            x: r * a.cos(),
            y: r * a.sin(),
            z,
        }
    }
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random(Some(-1.0), Some(1.0));
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_in_hemisphere(&self) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(&in_unit_sphere, self) > 0.0 {
            return in_unit_sphere;
        }
        return -in_unit_sphere;
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
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f32) -> Vec3 {
        return Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        };
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        return Vec3 {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        };
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f32) -> Vec3 {
        return Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        };
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
