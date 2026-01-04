use crate::vector::Vector;

const MAX_DEPTH: u32 = 32;
const MIN_SIZE: f32 = 0.01;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn midpoint(self: &Self, other: &Self) -> Self {
        Self {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }

    pub fn div(self: &Self, divisor: f32) -> Self {
        Self {
            x: (self.x / divisor),
            y: (self.y / divisor),
        }
    }

    pub fn mult_point(self: &Self, other: &Self) -> Self {
        Self {
            x: (self.x * other.x),
            y: (self.y * other.y),
        }
    }

    pub fn mult_scalar(self: &Self, other: f32) -> Self {
        Self {
            x: (self.x * other),
            y: (self.y * other),
        }
    }

    pub fn diff(self: &Self, other: &Self) -> Self {
        Self {
            x: (self.x - other.x).abs(),
            y: (self.y - other.y).abs(),
        }
    }

    pub fn add(self: &Self, other: &Self) -> Self {
        Self {
            x: (self.x + other.x),
            y: (self.y + other.y),
        }
    }

    pub fn distance(self: &Self, other: &Self) -> f32 {
        let dx: f32 = self.x - other.x;
        let dy: f32 = self.y - other.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }

    pub fn translate(&self, v: &Vector) -> Self {
        Self { x: self.x + v.x, y: self.y + v.y }
    }

    pub fn add_vec(&self, v: &Vector) -> Point {
        Point {x: self.x + v.x, y: self.y + v.y}
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

impl Rectangle {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1: p1, p2: p2 }
    }

    pub fn contains(&self, p: &Point) -> bool {
        self.p1.x <= p.x && self.p2.x >= p.x && self.p1.y <= p.y && self.p2.y >= p.y
    }

    pub fn center(&self) -> Point {
        Point {
            x: (self.p1.x + self.p2.x) * 0.5,
            y: (self.p1.y + self.p2.y) * 0.5,
        }
    }

    pub fn size(&self) -> Point {
        self.p1.diff(&self.p2)
    }

    pub fn move_rectangle(self, p: &Point) -> Self {
        Rectangle {
            p1: self.p1.add(p),
            p2: self.p2.add(p),
        }
    }

    pub fn intersects(&self, rect: &Rectangle) -> bool {
        self.p1.x <= rect.p1.x
            && self.p2.x >= rect.p1.x
            && self.p1.y <= rect.p2.y
            && self.p2.y >= rect.p2.y
    }

    pub fn distance_to_point(&self, p: &Point) -> f32 {
        let px: f32 = p.x;
        let py: f32 = p.y;

        let closest_x: f32 = self.p1.x.max(px.min(self.p2.x));
        let closest_y: f32 = self.p1.y.max(py.min(self.p2.y));

        let dx: f32 = px - closest_x;
        let dy: f32 = py - closest_y;

        dx.powi(2) + dy.powi(2)
    }
}

pub struct QuadTree {
    pub area: Rectangle,
    threshold: usize, // threshold will (should) always be 4, but setting default values isnt supported yet
    pub zones: [Option<Box<QuadTree>>; 4],
    pub elements: Vec<Point>,
    pub mass: f32,
    pub cm: Point,
}

impl QuadTree {
    pub fn new(rect: Rectangle) -> Self {
        QuadTree {
            area: rect,
            threshold: 4,
            zones: [None, None, None, None],
            elements: Vec::new(),
            mass: 0.0,
            cm: Point::zero(),
        }
    }

    pub fn has_zones(&self) -> bool {
        self.zones.iter().any(|z| z.is_some())
    }

    fn is_leaf(&self) -> bool {
        self.zones.iter().all(|z| z.is_none())
    }

    pub fn insert(&mut self, p: Point) {
        self.insert_depth(p, 0);
    }

    fn insert_depth(&mut self, p: Point, depth: u32) {
        // If point isn't inside this node, ignore (or handle however you want)
        if !self.area.contains(&p) {
            return;
        }

        let size = self.area.size();
        if depth >= MAX_DEPTH || size.x <= MIN_SIZE || size.y <= MIN_SIZE {
            self.elements.push(p);
            return;
        }

        if self.is_leaf() && self.elements.len() < self.threshold {
            self.elements.push(p);
            return;
        }

        if self.is_leaf() {
            self.subdivide(); // creates 4 children
            // reinsert existing points into children
            let old = std::mem::take(&mut self.elements);
            for q in old {
                self.insert_depth(q, depth + 1);
            }
        }

        // insert new point into a child
        for child in self.zones.iter_mut().filter_map(|z| z.as_deref_mut()) {
            if child.area.contains(&p) {
                child.insert_depth(p, depth + 1);
                return;
            }
        }

        // fallback: if point lies exactly on a boundary, keep it here
        self.elements.push(p);
    }

    fn subdivide(&mut self) {
        let p1 = self.area.p1;
        let p2 = self.area.p2;
        let mid = Point { x: (p1.x + p2.x) * 0.5, y: (p1.y + p2.y) * 0.5 };

        // NW, NE, SW, SE (pick your convention)
        let nw = Rectangle { p1: Point { x: p1.x, y: mid.y }, p2: Point { x: mid.x, y: p2.y } };
        let ne = Rectangle { p1: Point { x: mid.x, y: mid.y }, p2: Point { x: p2.x, y: p2.y } };
        let sw = Rectangle { p1: Point { x: p1.x, y: p1.y }, p2: Point { x: mid.x, y: mid.y } };
        let se = Rectangle { p1: Point { x: mid.x, y: p1.y }, p2: Point { x: p2.x, y: mid.y } };

        self.zones = [
            Some(Box::new(QuadTree::new(nw))),
            Some(Box::new(QuadTree::new(ne))),
            Some(Box::new(QuadTree::new(sw))),
            Some(Box::new(QuadTree::new(se))),
        ];
    }

    pub fn find_in_range(&self, rect: &Rectangle) -> Vec<Point> {
        let has_children = self.zones.iter().any(|z| z.is_some());

        if has_children {
            self.zones
                .iter()
                .filter_map(|z| z.as_deref())
                .filter(|child| rect.intersects(&child.area))
                .flat_map(|child| child.find_in_range(rect))
                .collect()
        } else {
            self.elements
                .iter()
                .copied()
                .filter(|p| rect.contains(p))
                .collect()
        }
    }

    pub fn find_nearest_neighbor(&self, p: Point) -> Option<Point> {
        let mut best: Option<Point> = None;
        let mut best_dist: f32 = f32::INFINITY;
        self.find_nearest_rec(p, &mut best, &mut best_dist);
        best
    }

    fn find_nearest_rec(&self, p: Point, best: &mut Option<Point>, best_dist: &mut f32) {
        if self.is_leaf() {
            for &element in &self.elements {
                // TS had `p !== el`. In Rust we usually skip same coordinates (or better: skip by ID).
                if element.x == p.x && element.y == p.y {
                    continue;
                }

                let d = p.distance(&element);
                if d < *best_dist {
                    *best_dist = d;
                    *best = Some(element);
                }
            }
            return;
        }

        // Collect existing children, sort by rect distance to p (closest first)
        let mut children: Vec<&QuadTree> = self.zones.iter().filter_map(|z| z.as_deref()).collect();
        children.sort_by(|a, b| {
            a.area
                .distance_to_point(&p)
                .partial_cmp(&b.area.distance_to_point(&p))
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for child in children {
            let rect_d = child.area.distance_to_point(&p);
            if rect_d < *best_dist {
                child.find_nearest_rec(p, best, best_dist);
            }
        }
    }
}
