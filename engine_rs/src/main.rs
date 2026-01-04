use beryllium::{
    Sdl, events,
    init::InitFlags,
    video::{CreateWinArgs, RendererFlags},
};
use std::time::{Duration, Instant};

use crate::draw::step_barnes_hut;
use crate::quadtree::{Point, QuadTree, Rectangle};
use crate::vector::Particle;

mod barnes_hut;
mod draw;
mod quadtree;
mod vector;

#[derive(Copy, Clone)]
struct Camera {
    cx: f32,
    cy: f32,
    ppu: f32,
}

fn world_to_screen(x: f32, y: f32, w: i32, h: i32, camera: Camera) -> [i32; 2] {
    let sx = (w as f32 * 0.5 + (x - camera.cx) * camera.ppu).round() as i32;
    let sy = (h as f32 * 0.5 - (y - camera.cy) * camera.ppu).round() as i32; // flip Y
    [sx, sy]
}

fn build_tree(particles: &[Particle], bounds: Rectangle) -> QuadTree {
    let mut qt = QuadTree::new(bounds);
    for p in particles {
        qt.insert(p.position);
    }
    qt
}

pub fn main() {
    let sdl = Sdl::init(InitFlags::EVERYTHING);

    let win = sdl
        .create_renderer_window(
            CreateWinArgs {
                title: "engine_rs",
                width: 1000,
                height: 800,
                allow_high_dpi: false,
                borderless: false,
                resizable: false,
            },
            RendererFlags::ACCELERATED_VSYNC,
        )
        .expect("couldn't create renderer window");

    // Hardcoded world bounds
    let bounds = Rectangle {
        p1: Point {
            x: -100.0,
            y: -75.0,
        },
        p2: Point { x: 100.0, y: 75.0 },
    };

    // Simple deterministic particles
    let n: usize = 500;
    let mut particles: Vec<Particle> = Vec::with_capacity(n);

    let mut seed: u32 = 123456789;
    let mut next_f32 = || {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        (seed as f32) / (u32::MAX as f32)
    };

    for _ in 0..n {
        let x = bounds.p1.x + (bounds.p2.x - bounds.p1.x) * next_f32();
        let y = bounds.p1.y + (bounds.p2.y - bounds.p1.y) * next_f32();
        let center: Point = Point { x: 0.0, y: 0.0 };
        let gm: f32 = 500.0; // same knob as your central gravity strength
        let eps2: f32 = 25.0; // softening (prevents insane speed near center)

        let dx = x - center.x;
        let dy = y - center.y;
        let r2 = dx * dx + dy * dy + eps2;
        let r = r2.sqrt();

        // circular orbit speed
        let v = (gm / r).sqrt();

        // perpendicular direction gives orbit
        let vx = -dy / r * v;
        let vy = dx / r * v;

        particles.push(Particle {
            position: Point { x, y },
            velocity: crate::vector::Vector { x: vx, y: vy },
            mass: 1.0,
        });
    }

    let dt: f32 = 1.0 / 30.0;
    let theta: f32 = 0.7;
    let g: f32 = 20.0;
    let eps2: f32 = 1e-3;

    // If you use VSYNC, you usually do NOT need manual sleeping,
    // but keeping a tiny cap is fine.
    let target_frame = Duration::from_secs_f32(1.0 / 60.0);
    let mut last_frame = Instant::now();

    // Reuse allocations
    let mut rects: Vec<[i32; 4]> = Vec::with_capacity(n);

    'main_loop: loop {
        // ---- events ----
        while let Some((event, _ts)) = sdl.poll_events() {
            match event {
                events::Event::Quit => break 'main_loop,
                events::Event::Key {
                    pressed: true,
                    keycode: _sdlk_escape,
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }

        // IMPORTANT: use the actual logical window size (esp. with high DPI)
        let (w, h) = win.get_window_size();

        // Compute camera every frame (cheap) so resizing / DPI changes won't break mapping
        let world_w = bounds.p2.x - bounds.p1.x;
        let world_h = bounds.p2.y - bounds.p1.y;
        let ppu = (w as f32 / world_w).min(h as f32 / world_h) * 0.95;

        let cam = Camera {
            cx: (bounds.p1.x + bounds.p2.x) * 0.5,
            cy: (bounds.p1.y + bounds.p2.y) * 0.5,
            ppu,
        };

        // ---- sim ----
        let root = build_tree(&particles, bounds);
        step_barnes_hut(&mut particles, &root, dt, theta, g, eps2);

        // ---- draw ----
        win.set_draw_color(255, 255, 255, 255).unwrap();
        win.clear().unwrap();

        // Draw particles as small filled squares (easier to see than 1px points)
        rects.clear();
        let r = 1; // radius -> 3x3
        for p in &particles {
            let [sx, sy] = world_to_screen(p.position.x, p.position.y, w, h, cam);

            // Skip if off-screen (avoids huge negative rects)
            if sx < -10 || sx > w + 10 || sy < -10 || sy > h + 10 {
                continue;
            }

            rects.push([sx - r, sy - r, 2 * r + 1, 2 * r + 1]);
            println!("{:?}", &p)
        }

        win.set_draw_color(0, 0, 0, 255).unwrap();
        win.fill_rects(&rects).unwrap();

        win.present();

        // ---- frame cap (optional if using VSYNC) ----
        let elapsed = last_frame.elapsed();
        if elapsed < target_frame {
            std::thread::sleep(target_frame - elapsed);
        }
        last_frame = Instant::now();
    }
}
