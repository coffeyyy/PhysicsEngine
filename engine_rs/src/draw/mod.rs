use crate::barnes_hut::{accel_toward_point, tree_force};
use crate::quadtree::{Point, QuadTree};
use crate::vector::Particle;
use std::ops::Add;

fn clamp_speed(v: &mut crate::vector::Vector, vmax: f32) {
    let s2 = v.x * v.x + v.y * v.y;
    let vmax2 = vmax * vmax;
    if s2 > vmax2 {
        let s = s2.sqrt();
        let k = vmax / s;
        v.x *= k;
        v.y *= k;
    }
}

fn resolve_collision(a: &mut Particle, b: &mut Particle, radius: f32, e: f32) {
    // vector from a -> b
    let dx = b.position.x - a.position.x;
    let dy = b.position.y - a.position.y;

    let dist2 = dx*dx + dy*dy;
    if dist2 == 0.0 {
        return;
    }

    let min_dist = 2.0 * radius;
    if dist2 > min_dist * min_dist {
        return; // not colliding
    }

    let dist = dist2.sqrt();
    let nx = dx / dist;
    let ny = dy / dist;

    // relative velocity along normal
    let rvx = b.velocity.x - a.velocity.x;
    let rvy = b.velocity.y - a.velocity.y;
    let vel_n = rvx * nx + rvy * ny;

    // Apply impulse only if moving *toward* each other
    if vel_n < 0.0 {
        let inv_ma = 1.0 / a.mass;
        let inv_mb = 1.0 / b.mass;

        let j = -(1.0 + e) * vel_n / (inv_ma + inv_mb);

        let imp_x = j * nx;
        let imp_y = j * ny;

        a.velocity.x -= imp_x * inv_ma;
        a.velocity.y -= imp_y * inv_ma;

        b.velocity.x += imp_x * inv_mb;
        b.velocity.y += imp_y * inv_mb;
    }

    // Positional correction (prevents "sinking" + huge impulses next frame)
    let penetration = min_dist - dist;
    let slop = 0.01;
    let percent = 0.8; // 0.2–0.8

    if penetration > slop {
        let inv_ma = 1.0 / a.mass;
        let inv_mb = 1.0 / b.mass;
        let inv_sum = inv_ma + inv_mb;

        let corr = (penetration - slop) / inv_sum * percent;

        a.position.x -= corr * nx * inv_ma;
        a.position.y -= corr * ny * inv_ma;

        b.position.x += corr * nx * inv_mb;
        b.position.y += corr * ny * inv_mb;
    }
}

pub fn step_barnes_hut(
    particles: &mut [Particle],
    root: &QuadTree,
    dt: f32,
    theta: f32,
    g: f32,
    eps2: f32,
) {
    // 1) forces
    let mut forces: Vec<Point> = Vec::with_capacity(particles.len());
    for part in particles.iter() {
        forces.push(tree_force(part.position, root, theta, g, eps2));
    }

    let center: Point = Point { x: 0.0, y: 0.0 };
    let gm: f32 = 500.0;
    let eps2_c: f32 = 25.0;

    // 2) velocity update only
    for (part, f_point) in particles.iter_mut().zip(forces.into_iter()) {
        let a_bh: crate::vector::Vector = crate::vector::Vector { x: f_point.x, y: f_point.y };
        let a_c: crate::vector::Vector  = accel_toward_point(part.position, center, gm, eps2_c);
        let a: crate::vector::Vector    = a_bh.add(a_c);

        part.velocity = part.velocity.add(a.mult_scalar(dt));
    }

    // 3) COLLISIONS GO HERE (once per pair)
    let radius = 1.0;
    let e = 0.99; // slightly elastic

    for i in 0..particles.len() {
        for j in (i + 1)..particles.len() {
            let (left, right) = particles.split_at_mut(j);
            let a = &mut left[i];
            let b = &mut right[0];
            resolve_collision(a, b, radius, e);
        }
    }

    // 4) clamp/damp + position update
    let vmax = 75.0;
    let damping = 0.99;

    for part in particles.iter_mut() {
        clamp_speed(&mut part.velocity, vmax);
        part.velocity = part.velocity.mult_scalar(damping);

        let dp = part.velocity.mult_scalar(dt);
        part.position = part.position.add_vec(&dp);
    }
}