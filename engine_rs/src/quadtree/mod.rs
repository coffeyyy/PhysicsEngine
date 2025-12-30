use std::cmp::{max, min};

pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
    }

    fn midpoint(self: &Self, other: &Self) -> Self {
        Self {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }

    fn div(self: &Self, other: &Self) -> Self {
        Self {
            x: (self.x / other.x),
            y: (self.y / other.y),
        }
    }

    fn mult(self: &Self, other: &Self) -> Self {
        Self {
            x: (self.x * other.x),
            y: (self.y * other.y),
        }
    }

    fn diff(self: &Self, other: &Self) -> Self {
        Self {
            x: (self.x - other.x).abs(),
            y: (self.y - other.y).abs(),
        }
    }

    fn add(self: &Self, other: &Self) -> Self {
        Self {
            x: (self.x + other.x),
            y: (self.y + other.y),
        }
    }

    fn distance(self: &Self, other: &Self) -> f32 {
        let dx: f32 = self.x - other.x;
        let dy: f32 = self.y - other.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn new(p1: Point, p2: Point) -> Self {
        Self { p1: p1, p2: p2 }
    }

    fn contains(&self, p: &Point) -> bool {
        self.p1.x <= p.x && self.p2.x >= p.x && self.p1.y <= p.y && self.p2.y >= p.y
    }

    fn size(&self) -> Point {
        self.p1.diff(&self.p2)
    }

    fn move_rectangle(self, p: &Point) -> Self {
        Rectangle {
            p1: self.p1.add(p),
            p2: self.p2.add(p),
        }
    }

    fn intersects(&self, rect: &Rectangle) -> bool {
        self.p1.x <= rect.p1.x
            && self.p2.x >= rect.p1.x
            && self.p1.y <= rect.p2.y
            && self.p2.y >= rect.p2.y
    }

    fn distance_to_point(&self, p: &Point) -> f32 {
        let px: f32 = p.x;
        let py: f32 = p.y;

        let closest_x: f32 = self.p1.x.max(px.min(self.p2.x));
        let closest_y: f32 = self.p1.y.max(py.min(self.p2.y));

        let dx: f32 = px - closest_x;
        let dy: f32 = py - closest_y;

        dx.powi(2) + dy.powi(2)
    }
}
