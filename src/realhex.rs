use std::ops::{ Add, Sub, Mul, Div };
use hexx::*;


#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct RealHex { pub x: f32, pub y: f32 }

impl RealHex {
    pub fn new(x: f32, y: f32) -> Self {
        RealHex { x, y }
    }
    
    pub fn to_hex(&self) -> Hex {
        Hex::new(self.x.round() as i32, self.y.round() as i32)
    }
}

impl From<&Hex> for RealHex {
    fn from(hex: &Hex) -> Self {
        RealHex { x: hex.x() as f32, y: hex.y() as f32 }
    }
}

impl Add for RealHex {
    type Output = Self;
    fn add(mut self, other: RealHex) -> Self::Output {
        self.x += other.x;
        self.y += other.y;
        self
    }
}

impl Sub for RealHex {
    type Output = Self;
    fn sub(mut self, other: RealHex) -> Self::Output {
        self.x -= other.x;
        self.y -= other.y;
        self
    }
}

impl Mul for RealHex {
    type Output = Self;
    fn mul(mut self, other: RealHex) -> Self::Output {
        self.x *= other.x;
        self.y *= other.y;
        self
    }
}

impl Div for RealHex {
    type Output = Self;
    fn div(mut self, other: RealHex) -> Self::Output {
        self.x /= other.x;
        self.y /= other.y;
        self
    }
}