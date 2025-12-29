mod VectorMath;

use std::ops::{Add, Sub, Mul};

mod VectorMath {

    pub struct Vector {
        x: f32,
        y: f32,
        z: f32,
    }

    impl VectorMath for Vector {
        fn length(a: &Vector) -> f32 {
            (
                (a.x ** 2),
                (a.y ** 2),
                (a.z ** 2),
            ).sqrt()
        }

        fn unit(a: &Vector) -> Vector {
            let magnitude: f32 = length(a);

            Vector{
                x: a.x / magnitude,
                y: a.y / magnitude,
                z: a.z / magnitude,
            }
        }

        fn dot(a: &Vector, b: &Vector) -> f32 {
            (
                (a.x * b.x)
                + (a.y * b.y)
                + (a.z * b.z)
            )
        }

        fn cross(a: &Vector, b: &Vector) -> Vector {
            Vector {
                x: a.y * b.z - a.z * b.y,
                a.z * b.x - a.x * b.z,
                a.x * b.y - a.y * b.x,
            }
        }
    }

    impl Add for Vector {
        type Output = Self;

        fn Add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl Sub for Vector {
        type Output = Self;

        fn Sub(self, other: Self) -> Self {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    impl Mul for Vector {
        type Output = Self;

        fn Mul(self, other: Self) -> Self {
            Self {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }
    }
}