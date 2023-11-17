pub use crate::vec3;
use std::{fmt, ops};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
    pub fn zero() -> Self {
        Default::default()
    }
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
    pub fn min(&self, arg: f32) -> Self {
        Self::new(self.x.min(arg), self.y.min(arg), self.z.min(arg))
    }
    pub fn max(&self, arg: f32) -> Self {
        Self::new(self.x.max(arg), self.y.max(arg), self.z.max(arg))
    }
}

impl From<f32> for Vec3 {
    fn from(arg: f32) -> Self {
        Self::new(arg, arg, arg)
    }
}

impl From<f64> for Vec3 {
    fn from(arg: f64) -> Self {
        Self::from(arg as f32)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", (self.x, self.y, self.z))
    }
}

impl Vec3 {
    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length(self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }
}

impl ops::Add<Self> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl<'a> std::iter::Sum<&'a Vec3> for Vec3 {
    fn sum<I>(iter: I) -> Vec3
    where
        I: Iterator<Item = &'a Vec3>,
    {
        iter.fold(Default::default(), |sum, x| sum + *x)
    }
}

impl std::iter::Sum<Vec3> for Vec3 {
    fn sum<I>(iter: I) -> Vec3
    where
        I: Iterator<Item = Vec3>,
    {
        iter.fold(Default::default(), |sum, x| sum + x)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, arg: f32) -> Self::Output {
        Vec3::new(self.x * arg, self.y * arg, self.z * arg)
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, arg: Vec3) -> Self::Output {
        arg * self
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, arg: f32) -> Self::Output {
        Vec3::new(self.x / arg, self.y / arg, self.z / arg)
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

#[macro_export]
macro_rules! vec3 {
    () => {
        $crate::math::Vec3::default()
    };

    ($expr:expr) => {
        $crate::math::Vec3::from($expr)
    };

    ($x:expr, $y:expr, $z:expr) => {
        $crate::math::Vec3::new($x as f32, $y as f32, $z as f32)
    };
}
