use std::collections::HashMap;

use instant::{Instant, Duration};
use macroquad::prelude::*;

use crate::{animation::Animation, Position, timer::Timer};

pub struct Player {
    position: Position,
    max_hp: f32,
    curr_hp: f32,
    curr_xp: f32,
    level: i32,
    flip_x: bool,
    speed_bonus: f32,
    iframes: Instant,
    inv_timer: Timer,
    curr_dmg: f32,
    anims: HashMap<String, Animation>,
}

const PLAYER_SPEED: f32 = 10.;

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        let mut idle_state_rects : Vec<Rect> = Vec::new();
        idle_state_rects.push(Rect::new(1., 1., 9., 9.));
        idle_state_rects.push(Rect::new(10., 1., 9., 9.));
        idle_state_rects.push(Rect::new(19., 1., 9., 9.));
        idle_state_rects.push(Rect::new(28., 1., 9., 9.));
    
        let idle_state_frame_lenghts : Vec<Duration> = vec![Duration::from_millis(200); idle_state_rects.len()];
    
        let idle_animation = Animation {
            frames: idle_state_rects.clone(),
            frame_length: idle_state_frame_lenghts.clone(),
            anim_duration: Duration::from_secs(0),
            current_frame: 0,
            current_frame_length: idle_state_frame_lenghts[0],
            repeating: true
        };
    
        let mut anims = HashMap::new();
        anims.insert("idle".to_string(), idle_animation);

        Player {
            position: Position { x, y },
            max_hp: 100.,
            curr_hp: 100.,
            curr_xp: 1.,
            level: 1,
            flip_x: false,
            speed_bonus: 1.,
            inv_timer: Timer::new(1800),
            iframes: Instant::now(),
            curr_dmg: 2.,
            anims
        }
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn move_me(&mut self, delta: f32) {
        let a = axis(is_key_down(KeyCode::Left), is_key_down(KeyCode::Right));
        let b = axis(is_key_down(KeyCode::Down), is_key_down(KeyCode::Up));
        let magnitude = (a.powi(2) + b.powi(2)).sqrt();
        let foo_x = a / magnitude;
        let foo_y = b / magnitude;
        if a != 0. || b != 0. {
            self.position.x += foo_x * delta * PLAYER_SPEED * self.speed_bonus;
            self.position.y -= foo_y * delta * PLAYER_SPEED * self.speed_bonus;
        }
    
        if a == -1. { self.flip_x = true }
        if a == 1. { self.flip_x = false }
    }

    pub fn draw(&self, texture: Texture2D, frame: Option<Rect>) {
        let mut color = WHITE;
        if self.inv_timer.value() != 1.0 {
            color = Color::new(1.0, 0., 0., 1.);
        }
        draw_texture_ex(
            texture, 
            self.position.x,
            self.position.y, 
            color,
    DrawTextureParams { 
                dest_size: Some(vec2(8., 8.)), 
                source: frame,
                flip_x: self.flip_x,
            ..Default::default()
        })
    }
}

pub fn axis(negative: bool, positive: bool) -> f32 {
    ((positive as i8) - (negative as i8)) as f32
}