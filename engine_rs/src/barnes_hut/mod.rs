use crate::quadtree::{Point, QuadTree};
use crate::vector::Vector;

fn inter_point_force(p: Point, q: Point, g: f32, eps2: f32) -> Point {
    let dx: f32 = q.x - p.x;
    let dy: f32 = q.y - p.y;

    let r2: f32 = dx * dx + dy * dy + eps2;
    let r: f32 = r2.sqrt();
    let inv_r3: f32 = 1.0 / (r2 * r);

    Point { x: g * dx * inv_r3, y: g * dy * inv_r3 }
}

fn force_point_to_mass(p: Point, cm: Point, mass: f32, g: f32, eps2: f32) -> Point {
    let dx: f32 = cm.x - p.x;
    let dy: f32 = cm.y - p.y;

    let radius2: f32 = dx * dx + dy * dy + eps2;
    let radius: f32 = radius2.sqrt();
    let inv_r3: f32 = 1.0 / (radius2 * radius);

    Point { x: g * mass * dx * inv_r3, y: g * mass * dy * inv_r3 }
}

pub fn accel_toward_point(pos: Point, center: Point, gm: f32, eps2: f32) -> Vector {
    let dx: f32 = center.x - pos.x;
    let dy: f32 = center.y - pos.y;

    let r2: f32 = dx * dx + dy * dy + eps2;
    let r: f32 = r2.sqrt();
    let inv_r3: f32 = 1.0 / (r2 * r);

    // acceleration = GM * r_vec / |r|^3
    Vector {
        x: gm * dx * inv_r3,
        y: gm * dy * inv_r3,
    }
}

pub fn tree_force(p: Point, node: &QuadTree, theta: f32, g: f32, eps2: f32) -> Point {
    let has_children: bool = node.zones.iter().any(|z| z.is_some());

    if !has_children {
        let mut force: Point = Point::zero();
        for &q in &node.elements {
            if q.x == p.x && q.y == p.y { continue; }
            force = force.add(&inter_point_force(p, q, g, eps2));
        }
        return force;
    }

    if node.mass == 0.0 {
        return Point::zero();
    }

    let size: Point = node.area.size();
    let d: f32 = size.x.max(size.y);

    let dx: f32 = node.cm.x - p.x;
    let dy: f32 = node.cm.y - p.y;
    let r2: f32 = dx*dx + dy*dy + eps2;
    let r: f32 = r2.sqrt();

    let contains_p: bool = node.area.contains(&p);

    if !contains_p && (d / r) < theta {
        return force_point_to_mass(p, node.cm, node.mass, g, eps2);
    }

    let mut force: Point = Point::zero();
    for child in node.zones.iter().filter_map(|z: &Option<Box<QuadTree>>| z.as_deref()) {
        force = force.add(&tree_force(p, child, theta, g, eps2));
    }
    force
}