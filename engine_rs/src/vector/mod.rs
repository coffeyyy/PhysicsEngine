use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {

    fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    fn equal_vectors(a: &Vector, b: &Vector) -> bool {
        a.x == b.x && a.y == b.y && a.z == b.z
    }

    fn length(a: &Vector) -> f32 {
        ((a.x).powi(2) + (a.y).powi(2) + (a.z).powi(2)).sqrt()
    }

    fn unit(a: &Vector) -> Vector {
        let magnitude: f32 = Self::length(a);

        Vector {
            x: a.x / magnitude,
            y: a.y / magnitude,
            z: a.z / magnitude,
        }
    }

    fn dot(a: &Vector, b: &Vector) -> f32 {
        (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
    }

    fn cross(a: &Vector, b: &Vector) -> Vector {
        Vector {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

pub struct Particle {
    pub position: Vector,
    pub velocity: Vector,
    pub mass: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        let a: Vector = Vector::new(1.0, 2.0, 3.0);
        let b: Vector = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(Vector::equal_vectors(&a, &b), true)
    }

    #[test]
    fn test_length() {
        let sample: Vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(Vector::length(&sample), 3.74165738677)
    }

    #[test]
    fn test_unit_vector() {
        let sample: Vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(
            Vector::unit(&sample),
            Vector {
                x: (1.0 / 3.74165738677),
                y: (2.0 / 3.74165738677),
                z: (3.0 / 3.74165738677),
            }
        )
    }

    #[test]
    fn test_dot() {
        let a: Vector = Vector::new(1.0, 2.0, 3.0);
        let b: Vector = Vector::new(3.0, 4.0, 5.0);
        assert_eq!(Vector::dot(&a, &b), 26.0)
    }

    #[test]
    fn test_cross() {
        let a: Vector = Vector::new(1.0, 0.0, -1.0);
        let b: Vector = Vector::new(2.0, 3.0, -1.0);

        assert_eq!(Vector::cross(&a, &b), 
        Vector{x: 3.0, y: -1.0, z: 3.0}
        )
    }
}
