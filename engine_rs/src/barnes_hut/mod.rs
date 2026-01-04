use crate::quadtree::{Point, QuadTree};
use crate::vector::{Particle, Vector};
use std::ops::Add;

fn inter_point_force(p: Point, q: Point, g: f32, eps2: f32) -> Point {
    /*

    */
    let dx: f32 = q.x - p.x;
    let dy: f32 = q.y - p.y;

    let r2: f32 = dx * dx + dy * dy + eps2;
    let r: f32 = r2.sqrt();
    let inv_r3: f32 = 1.0 / (r2 * r);

    Point {
        x: g * dx * inv_r3,
        y: g * dy * inv_r3,
    }
}

fn force_point_to_mass(p: Point, cm: Point, mass: f32, g: f32, eps2: f32) -> Point {
    /*

    */
    let dx: f32 = cm.x - p.x;
    let dy: f32 = cm.y - p.y;

    let radius2: f32 = dx * dx + dy * dy + eps2;
    let radius: f32 = radius2.sqrt();
    let inv_r3: f32 = 1.0 / (radius2 * radius);

    Point {
        x: g * mass * dx * inv_r3,
        y: g * mass * dy * inv_r3,
    }
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
            if q.x == p.x && q.y == p.y {
                continue;
            }
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
    let r2: f32 = dx * dx + dy * dy + eps2;
    let r: f32 = r2.sqrt();

    let contains_p: bool = node.area.contains(&p);

    if !contains_p && (d / r) < theta {
        return force_point_to_mass(p, node.cm, node.mass, g, eps2);
    }

    let mut force: Point = Point::zero();
    for child in node
        .zones
        .iter()
        .filter_map(|z: &Option<Box<QuadTree>>| z.as_deref())
    {
        force = force.add(&tree_force(p, child, theta, g, eps2));
    }
    force
}

fn clamp_speed(v: &mut Vector, vmax: f32) -> () {
    /*
    puts a speed limit on
     */
    let s2: f32 = v.x * v.x + v.y * v.y;
    let vmax2: f32 = vmax * vmax;
    if s2 > vmax2 {
        let s: f32 = s2.sqrt();
        let k: f32 = vmax / s;
        v.x *= k;
        v.y *= k;
    }
}

fn resolve_collision(a: &mut Particle, b: &mut Particle, radius: f32, e: f32) -> bool {
    // vector from a to b
    let dx: f32 = b.position.x - a.position.x;
    let dy: f32 = b.position.y - a.position.y;

    let dist2: f32 = dx * dx + dy * dy;
    if dist2 == 0.0 {
        return false;
    }

    let min_dist: f32 = 2.0 * radius;
    if dist2 > min_dist * min_dist {
        return false; // not colliding
    }

    let dist: f32 = dist2.sqrt();
    let nx: f32 = dx / dist;
    let ny: f32 = dy / dist;

    // relative velocity along normal
    let rvx: f32 = b.velocity.x - a.velocity.x;
    let rvy: f32 = b.velocity.y - a.velocity.y;
    let vel_n: f32 = rvx * nx + rvy * ny;

    // Apply impulse only if moving toward each other
    if vel_n < 0.0 {
        let inv_ma: f32 = 1.0 / a.mass;
        let inv_mb: f32 = 1.0 / b.mass;

        let j: f32 = -(1.0 + e) * vel_n / (inv_ma + inv_mb);

        let imp_x: f32 = j * nx;
        let imp_y: f32 = j * ny;

        a.velocity.x -= imp_x * inv_ma;
        a.velocity.y -= imp_y * inv_ma;

        b.velocity.x += imp_x * inv_mb;
        b.velocity.y += imp_y * inv_mb;
    }

    // Positional correction (prevents "sinking" + huge impulses next frame)
    let penetration: f32 = min_dist - dist;
    let slop: f32 = 0.01;
    let percent: f32 = 0.8;

    if penetration > slop {
        let inv_ma: f32 = 1.0 / a.mass;
        let inv_mb: f32 = 1.0 / b.mass;
        let inv_sum: f32 = inv_ma + inv_mb;

        let corr: f32 = (penetration - slop) / inv_sum * percent;

        a.position.x -= corr * nx * inv_ma;
        a.position.y -= corr * ny * inv_ma;

        b.position.x += corr * nx * inv_mb;
        b.position.y += corr * ny * inv_mb;
    }

    return true;
}

pub fn step_barnes_hut(
    particles: &mut [Particle],
    root: &QuadTree,
    dt: f32,
    theta: f32,
    g: f32,
    eps2: f32,
) -> u32 {
    /*
    
    */

    // forces
    let mut forces: Vec<Point> = Vec::with_capacity(particles.len());
    for part in particles.iter() {
        forces.push(tree_force(part.position, root, theta, g, eps2));
    }

    let center: Point = Point { x: 0.0, y: 0.0 };
    let gm: f32 = 500.0;
    let eps2_c: f32 = 25.0;

    // velocity update only
    for (part, f_point) in particles.iter_mut().zip(forces.into_iter()) {
        let a_bh: crate::vector::Vector = crate::vector::Vector {
            x: f_point.x,
            y: f_point.y,
        };
        let a_c: crate::vector::Vector = accel_toward_point(part.position, center, gm, eps2_c);
        let a: crate::vector::Vector = a_bh.add(a_c);

        part.velocity = part.velocity.add(a.mult_scalar(dt));
    }

    let mut collision_count: u32 = 0;
    let radius: f32 = 1.0;
    let e: f32 = 0.99; // elasticity - how much energy is transfered during collisions

    for i in 0..particles.len() {
        for j in (i + 1)..particles.len() {
            let (left, right) = particles.split_at_mut(j);
            let a: &mut Particle = &mut left[i];
            let b: &mut Particle = &mut right[0];
            resolve_collision(a, b, radius, e);
            if resolve_collision(a, b, radius, e) {
                collision_count += 1;
            }
        }
    }

    // clamp & damp
    let vmax: f32 = 75.0;
    let damping: f32 = 0.99;

    for part in particles.iter_mut() {
        clamp_speed(&mut part.velocity, vmax);
        part.velocity = part.velocity.mult_scalar(damping);

        let dp: Vector = part.velocity.mult_scalar(dt);
        part.position = part.position.add_vec(&dp);
    }
    collision_count
}

// helpers to draw the sim
#[derive(Copy, Clone)]
pub struct Camera {
    cx: f32,
    cy: f32,
    ppu: f32,
}

pub fn world_to_screen(x: f32, y: f32, w: i32, h: i32, camera: Camera) -> [i32; 2] {
    /*
    create the space for the simulation to happen
     */
    let sx: i32 = (w as f32 * 0.5 + (x - camera.cx) * camera.ppu).round() as i32;
    let sy: i32 = (h as f32 * 0.5 - (y - camera.cy) * camera.ppu).round() as i32; // flip Y
    [sx, sy]
}
