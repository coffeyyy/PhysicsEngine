use crate::quadtree::{Point, QuadTree};
use crate::vector::Particle;

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

fn compute_mass(node: &QuadTree) -> (f32, Point) {
    let is_leaf: bool = node.zones.iter().all(|z| z.is_none());

    

    if is_leaf {
        let total_mass: f32 = node.elements.len() as f32;

        if total_mass == 0.0 {
            return (0.0, node.area.center());
        }

        let mut sum: Point = Point::zero();
        for point in &node.elements {
            sum = sum.add(point);
        }

        let cm: Point = sum.div(total_mass);
        return (total_mass, cm)
    }

    let mut child_mass: f32 = 0.0;
    let mut weighted_sum: Point = Point::zero();

    for child in node.zones.iter().filter_map(|z| z.as_deref()) {
        let (m, cm): (f32, Point) = compute_mass(child);
        child_mass += m;

        weighted_sum = weighted_sum.add(&cm.mult_scalar(m));
    }

    if child_mass == 0.0 {
        return (0.0, node.area.center());
    }

    let cm: Point = weighted_sum.div(child_mass);
    return (child_mass, cm);
}

pub fn accel_toward_point(pos: Point, center: Point, gm: f32, eps2: f32) -> crate::vector::Vector {
    let dx = center.x - pos.x;
    let dy = center.y - pos.y;

    let r2 = dx * dx + dy * dy + eps2;
    let r = r2.sqrt();
    let inv_r3 = 1.0 / (r2 * r);

    // acceleration = GM * r_vec / |r|^3
    crate::vector::Vector {
        x: gm * dx * inv_r3,
        y: gm * dy * inv_r3,
    }
}

pub fn tree_force(p: Point, node: &QuadTree, theta: f32, g: f32, eps2: f32) -> Point {
    let has_children: bool = node.zones.iter().any(|z| z.is_some());

    if !has_children {
        let mut force: Point = Point::zero();

        for &q in &node.elements {
            if q.x == p.x && q.y == p.y {
                continue;
            }

            force = force.add(&inter_point_force(p, q, g, eps2));
        }
        return force;
    }

    let (mass, cm) = compute_mass(node);

    if mass == 0.0 {
        return Point::zero();
    }

    let size: Point = node.area.size();
    let d: f32 = size.x.max(size.y);

    let dx: f32 = cm.x - p.x;
    let dy: f32 = cm.y - p.y;
    let radius2: f32 = dx * dx + dy * dy + eps2;

    let radius: f32 = radius2.sqrt();

    let contains_p: bool = node.area.contains(&p);

    if !contains_p && (d / radius) < theta {
        return force_point_to_mass(p, cm, mass, g, eps2);
    }

    let mut force: Point = Point::zero();

    for child in node.zones.iter().filter_map(|z| z.as_deref()) {
        force = force.add(&tree_force(p, child, theta, g, eps2))
    }

    force
}