use std::ops::{Add, Mul, Sub};

use crate::quadtree::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {

    fn new(x: f32, y: f32) -> Vector {
        Vector { x: x, y: y }
    }

    fn equal_vectors(a: &Vector, b: &Vector) -> bool {
        a.x == b.x && a.y == b.y 
    }

    fn length(a: &Vector) -> f32 {
        ((a.x).powi(2) + (a.y).powi(2)).sqrt()
    }

    fn unit(a: &Vector) -> Vector {
        let magnitude: f32 = Self::length(a);

        Vector {
            x: a.x / magnitude,
            y: a.y / magnitude,
        }
    }

    pub fn mult_scalar(&self, scalar: f32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    fn dot(a: &Vector, b: &Vector) -> f32 {
        (a.x * b.x) + (a.y * b.y)
    }

    fn cross(a: &Vector, b: &Vector) -> f32 {
        a.x * b.y - a.y * b.x
    }
    
    pub fn from_point(p: &Point) -> Self {
        Vector { x: p.x, y: p.y }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub position: Point,
    pub velocity: Vector,
    pub mass: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        let a: Vector = Vector::new(1.0, 2.0);
        let b: Vector = Vector::new(1.0, 2.0);

        assert_eq!(Vector::equal_vectors(&a, &b), true)
    }

    #[test]
    fn test_length() {
        let sample: Vector = Vector::new(1.0, 2.0);
        assert_eq!(Vector::length(&sample), 3.74165738677)
    }

    #[test]
    fn test_unit_vector() {
        let sample: Vector = Vector::new(1.0, 2.0);
        assert_eq!(
            Vector::unit(&sample),
            Vector {
                x: (1.0 / 3.74165738677),
                y: (2.0 / 3.74165738677),
            }
        )
    }

    #[test]
    fn test_dot() {
        let a: Vector = Vector::new(1.0, 2.0);
        let b: Vector = Vector::new(3.0, 4.0);
        assert_eq!(Vector::dot(&a, &b), 26.0)
    }

    #[test]
    fn test_cross() {
        let a: Vector = Vector::new(1.0, 0.0);
        let b: Vector = Vector::new(2.0, 3.0);

        assert_eq!(Vector::cross(&a, &b), 3.0)
    }
}
