use std::fmt;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Coordinate = Vec3;
pub type Color = Vec3;

impl Color {
    pub fn print(&self) {
        println!(
            "{} {} {}",
            (255.999 * self.x) as u8,
            (255.999 * self.y) as u8,
            (255.999 * self.z) as u8
        );
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
    pub fn length_squared(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }
    pub fn dot(&self, vec: &Vec3) -> f32 {
        return (self.x * vec.x) + (self.y * vec.y) + (self.z + vec.z);
    }
    pub fn cross(&self, vec: &Vec3) -> Self {
        return Vec3 {
            x: (self.y * vec.z) - (self.z * vec.y),
            y: (self.z * vec.x) - (self.x * vec.z),
            z: (self.x * vec.y) - (self.y * vec.x),
        };
    }
    pub fn unit_vec(&self) -> Self {
        return *self / self.length();
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
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
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
