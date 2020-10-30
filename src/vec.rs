use serde::{Serialize, Deserialize};
use std::ops::{Add, Sub, Mul, AddAssign, SubAssign};
use std::iter::Sum;
use std::fmt;


#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Vec2(f64, f64);


impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2(x, y)
    }

    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }

    pub fn norm_sq(&self) -> f64 {
        self.0*self.0 + self.1*self.1
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = Vec2(self.0 + other.0, self.1 + other.1);
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2(self.0 - other.0, self.1 - other.1)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        *self = Vec2(self.0 - other.0, self.1 - other.1);
    }
}

impl Mul for Vec2 {
    type Output = f64;

    fn mul(self, other: Vec2) -> f64 {
        self.0*other.0 + self.1*other.1
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: f64) -> Vec2 {
        Vec2(self.0*other, self.1*other)
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2(self*other.0, self*other.1)
    }
}

impl Sum for Vec2 {
    fn sum<I: Iterator<Item=Vec2>>(iter: I) -> Vec2 {
        iter.fold(Vec2(0.0, 0.0), Add::add)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}
