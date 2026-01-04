use beryllium::{
    Sdl, events,
    init::InitFlags,
    video::{CreateWinArgs, RendererFlags},
};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;
use std::time::{Duration};

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
    let sx: i32 = (w as f32 * 0.5 + (x - camera.cx) * camera.ppu).round() as i32;
    let sy: i32 = (h as f32 * 0.5 - (y - camera.cy) * camera.ppu).round() as i32; // flip Y
    [sx, sy]
}

fn build_tree(particles: &[Particle], bounds: Rectangle) -> QuadTree {
    let mut qt: QuadTree = QuadTree::new(bounds);
    for p in particles {
        qt.insert(p.position);
    }
    qt
}

pub fn main() {
    let sdl: Sdl = Sdl::init(InitFlags::EVERYTHING);

    let mut csv = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("collisions_per_sec.csv")
        .expect("failed to open csv");

    writeln!(csv, "t_sec,collisions_per_sec").unwrap();

    let start_time = Instant::now();

    let win: beryllium::video::RendererWindow = sdl
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
    let bounds: Rectangle = Rectangle {
        p1: Point {
            x: -100.0,
            y: -75.0,
        },
        p2: Point { x: 100.0, y: 75.0 },
    };

    let n: usize = 1000; // # of particles
    let mut particles: Vec<Particle> = Vec::with_capacity(n);

    let mut coll_accum: u64 = 0;
    let mut coll_timer = Instant::now();

    // generate random starting positions
    let mut seed: u32 = 123456789;
    let mut next_f32 = || {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        (seed as f32) / (u32::MAX as f32)
    };

    for _ in 0..n {
        let x: f32 = bounds.p1.x + (bounds.p2.x - bounds.p1.x) * next_f32();
        let y: f32 = bounds.p1.y + (bounds.p2.y - bounds.p1.y) * next_f32();
        let center: Point = Point { x: 0.0, y: 0.0 };
        let gm: f32 = 500.0; // Mass of the central point
        let eps2: f32 = 25.0; // softening (prevents insane speed near center)

        let dx: f32 = x - center.x;
        let dy: f32 = y - center.y;
        let r2: f32 = dx * dx + dy * dy + eps2;
        let r: f32 = r2.sqrt();

        // circular orbit speed
        let v: f32 = (gm / r).sqrt();

        // perpendicular direction gives orbit
        let vx: f32 = -dy / r * v;
        let vy: f32 = dx / r * v;

        particles.push(Particle {
            position: Point { x, y },
            velocity: crate::vector::Vector { x: vx, y: vy },
            mass: 1.0,
        });
    }

    let dt: f32 = 1.0 / 15.0;
    let theta: f32 = 0.7;
    let g: f32 = 20.0;
    let eps2: f32 = 1e-3;

    let target_frame: Duration = Duration::from_secs_f32(1.0 / 60.0);
    let mut last_frame: Instant = Instant::now();

    // Reuse allocations
    let mut rects: Vec<[i32; 4]> = Vec::with_capacity(n);

    'main_loop: loop {
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

        let (w, h) = win.get_window_size();

        // Compute camera every frame (cheap) so resizing / DPI changes won't break mapping
        let world_w: f32 = bounds.p2.x - bounds.p1.x;
        let world_h: f32 = bounds.p2.y - bounds.p1.y;
        let ppu: f32 = (w as f32 / world_w).min(h as f32 / world_h) * 0.95;

        let cam: Camera = Camera {
            cx: (bounds.p1.x + bounds.p2.x) * 0.5,
            cy: (bounds.p1.y + bounds.p2.y) * 0.5,
            ppu,
        };

        let root = build_tree(&particles, bounds);
        let c = step_barnes_hut(&mut particles, &root, dt, theta, g, eps2);
        coll_accum += c as u64;

        let elapsed = coll_timer.elapsed().as_secs_f32();
        if elapsed >= 1.0 {
            let cps = coll_accum as f32 / elapsed;
            let t = start_time.elapsed().as_secs_f32();

            writeln!(csv, "{:.3},{:.3}", t, cps).unwrap();
            csv.flush().unwrap(); // optional, but nice for live logging

            coll_accum = 0;
            coll_timer = Instant::now();
        }

        win.set_draw_color(255, 255, 255, 255).unwrap();
        win.clear().unwrap();

        // Draw particles as small filled squares (makes it easier to see)
        rects.clear();
        let r: i32 = 1; // radius -> 3x3
        for p in &particles {
            let [sx, sy] = world_to_screen(p.position.x, p.position.y, w, h, cam);

            // Skip if off-screen
            if sx < -10 || sx > w + 10 || sy < -10 || sy > h + 10 {
                continue;
            }

            rects.push([sx - r, sy - r, 2 * r + 1, 2 * r + 1]);
        }

        win.set_draw_color(0, 0, 0, 255).unwrap();
        win.fill_rects(&rects).unwrap();

        win.present();

        // frame limit
        let elapsed = last_frame.elapsed();
        if elapsed < target_frame {
            std::thread::sleep(target_frame - elapsed);
        }
        last_frame = Instant::now();
    }
}
