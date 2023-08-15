use macroquad::{prelude::*};

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
    texture: Option<Texture2D>,
    pub active: bool
}

pub trait ParticleType {
    fn new(&self, x: f32, y: f32) -> Particle;
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
            texture: None,
            active: true
        }
    }
}

pub struct ShotParticle {}

impl ParticleType for ShotParticle {
    fn new(&self, x: f32, y: f32) -> Particle {
        let mut particle = Particle {
            ..Default::default()
        };
        particle.x = x + (rand::gen_range(0.5, 1.) - 0.5) * 2.;
        particle.y = y + (rand::gen_range(0.5, 1.) - 0.5) * 2.;

        particle        
    }
}

pub struct EnemyShotParticle {}

impl ParticleType for EnemyShotParticle {
    fn new(&self, x: f32, y: f32) -> Particle {
        let mut particle = Particle {
            ..Default::default()
        };
        particle.color_start = Color::from_rgba(168, 132, 243, 255);
        particle.color_end = Color::new(0.2, 0.2, 0.2, 0.1); 
        particle.x = x + (rand::gen_range(0.5, 1.) - 0.5) * 2.;
        particle.y = y + (rand::gen_range(0.5, 1.) - 0.5) * 2.;

        particle        
    }
}

pub struct PlayerDashParticle {
    pub texture: Texture2D
}

impl ParticleType for PlayerDashParticle {
    fn new(&self, x: f32, y: f32) -> Particle {
        let mut particle = Particle {
            ..Default::default()
        };
        particle.velocity_start = vec2(1., 0.); 
        particle.velocity_end = vec2(1., 0.);
        particle.color_start = Color::new(1., 1., 1., 0.3); 
        particle.color_end = Color::new(0.2, 0.2, 0.2, 0.1); 
        particle.x = x + (rand::gen_range(0.5, 1.) - 0.5) * 0.1;
        particle.y = y + (rand::gen_range(0.5, 1.) - 0.5) * 0.5;
        particle.texture = Some(self.texture);

        particle        
    }
}

pub struct IntroParticle {}

impl ParticleType for IntroParticle {
    fn new(&self, x: f32, y: f32) -> Particle {
        let mut particle = Particle {
            ..Default::default()
        };
        particle.lifetime = Timer::new(10000);
        particle.velocity_start = vec2(1., 5.); 
        particle.velocity_end = vec2(1., 5.);
        particle.color_start = Color::new(1., 1., 1., 0.5); 
        particle.color_end = Color::new(0.2, 0.2, 0.2, 0.1);
        particle.size_start = 1.5 + (rand::gen_range(0.5, 1.) - 0.5) * 3.0; 
        particle.size_end = 0.2 + (rand::gen_range(0.5, 1.) - 0.5) * 3.0;
        particle.x = x + (rand::gen_range(0.5, 1.) - 0.5) * 0.1 * 5000.;
        particle.y = y + (rand::gen_range(0.5, 1.) - 0.5) * 0.5 * 5000.;

        particle        
    }
}

pub fn spawn_particle(particles: &mut Vec<Particle>, x: f32, y: f32, particle_type: Box<dyn ParticleType>) {
    let p = particle_type.new(x, y);
    particles.push(p);
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
            if let Some(texture) = particle.texture {
                let color = lerp_color(particle.color_start, particle.color_end, elapsed);
                draw_texture_ex(
                    texture, 
                    particle.x,
                    particle.y,
                    color,
            DrawTextureParams { 
                        dest_size: Some(vec2(8., 8.)), 
                        source: Some(Rect::new(1., 1., 9., 9.)),
                    ..Default::default()
                });
            }  else {
                let size = lerp(particle.size_start, particle.size_end, elapsed);
                let color = lerp_color(particle.color_start, particle.color_end, elapsed);
                draw_rectangle(particle.x, particle.y, size, size, color);
            }
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

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

pub fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    a * (1.0 - t) + b * t
}