use keyframe::{Keyframe, functions::{EaseOut, EaseInOut}};
use macroquad::prelude::*;

use crate::{Position, Collider, timer::Timer, tween::Tween};
use super::{col, get_dir_, dist};

#[derive(Clone)]
pub struct Enemies {
    pub position: Position,
    pub collider: Collider,
    pub alive: bool,
    pub hp: f32,
    pub tween: Tween
}

impl Enemies {
    pub fn new(x: f32, y: f32) -> Self {
        let tween = Tween::from_keyframes(
            vec![
                Keyframe::new(0.0, 0.0, EaseOut),
                Keyframe::new(20.0, 0.2, EaseOut),
            ],
            0,
            1,
            false,
        );
        
        Enemies {
            position: Position {
                x,
                y,
            },
            collider: Collider { 
                x: 72.,
                y: 90.,
                width: 8, 
                height: 8,
                radius: 4. 
            },
            hp: 2.,
            alive: true,
            tween
        }
    }
}

pub fn update_enemies_position(enemies: &mut Vec<Enemies>, x: &mut f32, y: &mut f32) {
    let delta = get_frame_time();
    let player_vec: Vec2 = Vec2::new(*x, *y);

    for e in enemies.iter_mut() {
        // x.atan2(other);
        if e.hp > 0. {
            let enem_vec = Vec2::new(e.position.x, e.position.y);
            let dir = enem_vec - player_vec;
            e.position.x -= dir.x * delta * 0.3; 
            e.position.y -= dir.y * delta * 0.3;
        }
    }
}

pub fn update_enemies_colliding(enemies: &mut Vec<Enemies>, x: &f32, y: &f32, hp: &mut f32, player_inv_timer: &mut Timer) {
    for e in enemies.iter() {
        let player_pos = Position {
            x: *x,
            y: *y
        };
        if player_inv_timer.value() == 1.0 {
            if col(player_pos, e.position, 8.) {
                println!("colliding with player");
                damage_player(hp);
                player_inv_timer.restart();
            }
        }
    }
}

fn damage_player(hp: &mut f32) {
    *hp -= 10.;
}

pub fn update_enemies_pushing(enemies: &mut Vec<Enemies>) {
    if enemies.len() > 0 {
        for i in 0..enemies.len() - 1 {
            for j in i+1..enemies.len() {
                // let r = enemies[i].collider.radius + enemies[j].collider.radius;
                let r = 8.;
                if col(enemies[i].position, enemies[j].position, r) {
                    // println!("enemies colliding!");
                    let dist = dist(enemies[i].position, enemies[j].position, 10.);
                    let dir = get_dir_(enemies[i].position, enemies[j].position);
                    let dif = r - dist;
                    enemies[i].position.x += dir.cos()*dif;
                    enemies[i].position.y += dir.sin()*dif;
                }
            }
        }
    }
}

pub fn draw_enemies(texture: Texture2D, enemies: &mut Vec<Enemies>, x: &mut f32, y: &mut f32) {
    for e in enemies.iter() {
        let mut flip = false;
        if e.position.x > *x {
            flip = true
        }
        draw_texture_ex(
            texture, 
            e.position.x,
            e.position.y,
            WHITE,
    DrawTextureParams { 
                dest_size: Some(vec2(8., 8.)), 
                source: Some(Rect::new(
                    20.,
                    2.,
                    8.,
                    8.,
                )),
                flip_x: flip,
            ..Default::default()
        });
    } 
}

pub fn update_dead_enemies(enemies: &mut Vec<Enemies>, x: &mut f32) {
    let delta = get_frame_time();
    for e in enemies.iter_mut() {
        e.tween.update();
        if e.position.x > *x {
            e.position.x += e.tween.value() * delta;
        } else {
            e.position.x -= e.tween.value() * delta;
        }
        if e.tween.finished() {
            e.alive = false;
        }
    }
}

pub fn draw_dead_enemies(texture: Texture2D, enemies: &mut Vec<Enemies>, x: &mut f32, y: &mut f32) {
    for e in enemies.iter() {
        if e.alive {
            let mut flip = false;
            if e.position.x > *x {
                flip = true
            }
            draw_texture_ex(
                texture, 
                e.position.x,
                e.position.y,
                WHITE,
        DrawTextureParams { 
                    dest_size: Some(vec2(8., 8.)), 
                    source: Some(Rect::new(
                        20.,
                        2.,
                        8.,
                        8.,
                    )),
                    flip_x: flip,
                ..Default::default()
            });
        }
    } 
}

pub fn draw_enemies_collider(enemies: &mut Vec<Enemies>) {
    for e in enemies.iter() {
        draw_circle(
            e.position.x + 4., 
            e.position.y + 4., 
            e.collider.radius, 
            Color::from_rgba(255, 0, 0, 60)
        );
    }
}

pub fn spawn_enemies(enemies: &mut Vec<Enemies>, player_pos_x: &f32, player_pos_y: &f32) {
    // get a random position away from the player
    // add an enemy to that position
    let direction = rand::gen_range(-1, 2) as f32;
    let random;
    // let mut rng = ::rand::thread_rng();

    match rand::gen_range(0, 2) {
        0 => random = -1.,
        _ => random = 1.,
    }

    let _rad = 60. + (rand::gen_range(0., 33.) as f32).floor();
    let x = player_pos_x + direction.cos() * _rad * random;
    let y = player_pos_y + direction.sin() * _rad * random;

    enemies.push(Enemies::new(x, y));
}
