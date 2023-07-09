use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn write_color(v: Vec3) -> () {
    let ir: i32 = (255.99 * v.x()) as i32;
    let ig: i32 = (255.99 * v.y()) as i32;
    let ib: i32 = (255.99 * v.z()) as i32;
    println!("{} {} {}", ir, ig, ib);
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }
    pub fn z(self) -> f64 {
        self.z
    }
    pub fn squared_length(self) -> f64 {
        self.x().powf(2.0) + self.y().powf(2.0) + self.z().powf(2.0)
    }

    pub fn length(self) -> f64 {
        (self.squared_length()).sqrt()
    }

    pub fn unit(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(self, v: Vec3) -> f64 {
        self.x() * v.x() + self.y() * v.y() + self.z() * v.z()
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.y() * v.z() - self.z() * v.y(),
            self.z() * v.x() - self.x() * v.z(),
            self.x() * v.y() - self.y() * v.x(),
        )
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z),
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
            z: self.z.sub(rhs.z),
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(rhs.x * self, self * rhs.y, self * rhs.z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x, self * v.y, self * v.z)
    }
}
