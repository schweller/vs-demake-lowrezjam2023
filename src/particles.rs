use macroquad::prelude::*;

use crate::timer::Timer;

pub struct Particle {
    lifetime: Timer,
    velocity_start: Vec2,
    velocity_end: Vec2,
    size_start: f32,
    size_end: f32,
    color_start: Color,
    color_end: Color,
    x: f32,
    y: f32,
    pub active: bool
}

impl Default for Particle {
    fn default() -> Self {
        Particle { 
            lifetime: Timer::new(500), 
            velocity_start: vec2(5., 0.), 
            velocity_end: vec2(5., 0.), 
            size_start: 1.5 + (rand::gen_range(0.5, 1.) - 0.5), 
            size_end: 0.2 + (rand::gen_range(0.5, 1.) - 0.5), 
            // color_start: Color::new(1., 0.35, 0.0, 1.),
            color_start: Color::from_rgba(251, 185, 84, 255), 
            color_end: Color::new(0.2, 0.2, 0.2, 0.1), 
            x: 50., 
            y: 50.,
            active: true
        }
    }
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

pub fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    a * (1.0 - t) + b * t
}

pub fn spawn_particle(particles: &mut Vec<Particle>, x: f32, y: f32) {
    particles.push(
        Particle {
            x: x + (rand::gen_range(0.5, 1.) - 0.5) * 2.,
            y: y + (rand::gen_range(0.5, 1.) - 0.5) * 2.,
            ..Default::default()
        }
    )
}

pub fn update_particles(particles: &mut Vec<Particle>) {
    let delta = get_frame_time();
    for particle in particles.iter_mut() {
        if particle.active {
            if particle.lifetime.finished() {
                particle.active = false;
            }
            let elapsed = particle.lifetime.elapsed().as_secs_f32() / particle.lifetime.duration.as_secs_f32();
            let velocity = lerp_vec2(particle.velocity_start, particle.velocity_end, elapsed);
            particle.x += velocity.extend(0.0).x * delta;
            particle.y += velocity.extend(0.0).y * delta;
        }
    }
}

pub fn draw_particles(particles: &mut Vec<Particle>) {
    for particle in particles.iter() {
        if particle.active {
            let elapsed = particle.lifetime.elapsed().as_secs_f32() / particle.lifetime.duration.as_secs_f32();
            let size = lerp(particle.size_start, particle.size_end, elapsed);
            let color = lerp_color(particle.color_start, particle.color_end, elapsed);
            draw_rectangle(particle.x, particle.y, size, size, color);
        }
    }
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        lerp(a.r, b.r, t),
        lerp(a.g, b.g, t),
        lerp(a.b, b.b, t),
        lerp(a.a, b.a, t),
    )
}


