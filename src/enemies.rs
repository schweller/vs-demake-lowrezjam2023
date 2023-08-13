use std::collections::HashMap;

use instant::Duration;
use keyframe::{Keyframe, functions::{EaseOut, EaseInOut}};
use macroquad::prelude::*;

use crate::{Position, Collider, timer::Timer, tween::Tween, animation::Animation, Bullet, particles::{spawn_particle, ShotParticle, Particle, EnemyShotParticle}, damage_popup::DamagePopup};
use super::{col, get_dir_, dist};

#[derive(Clone)]
pub struct Enemies {
    pub position: Position,
    pub collider: Collider,
    pub alive: bool,
    pub hp: f32,
    pub tween: Tween,
    pub anims: HashMap<String, Animation>,
    pub curr_frame: Option<Rect>
}

pub struct DeadEnemy {
    pub position: Position,
    pub move_tween: Tween,
    pub opacity_tween: Tween,
    pub active: bool,
    pub curr_frame: Option<Rect>
}

impl DeadEnemy {
    pub fn new(x: f32, y: f32, frame: Option<Rect>) -> Self {
        let move_tween = Tween::from_keyframes(
            vec![
                Keyframe::new(0.0, 0.0, EaseOut),
                Keyframe::new(20.0, 0.2, EaseOut),
            ],
            0,
            1,
            false,
        );
        let opacity_tween = Tween::from_keyframes(
            vec![
                Keyframe::new(1.0, 0.0, EaseOut),
                Keyframe::new(0.0, 0.2, EaseOut),
            ],
            0,
            1,
            false,
        );

        DeadEnemy { position: Position { x, y }, move_tween, opacity_tween, active: true, curr_frame: frame }
    }
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
            tween,
            anims,
            curr_frame: Some(Rect::new(1., 1., 9., 9.))
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

pub fn update_enemies_colliding(
    enemies: &mut Vec<Enemies>, 
    x: &f32, y: &f32, hp: &mut f32, 
    player_is_dashing: &bool,
    player_inv_timer: &mut Timer,
    screen_shake_amount: &mut f32) 
{
    for e in enemies.iter() {
        let player_pos = Position {
            x: *x,
            y: *y
        };
        if !*player_is_dashing && player_inv_timer.value() == 1.0 {
            if col(player_pos, e.position, 8.) {
                println!("colliding with player");
                damage_player(hp);
                *screen_shake_amount += 4.0;
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
    for e in enemies.iter_mut() {
        let frame = e.anims.get_mut("idle").unwrap().get_animation_source(Duration::from_secs_f32(get_frame_time()));
        e.curr_frame = frame;
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
                dest_size: Some(vec2(9., 9.)), 
                source: frame,
                flip_x: flip,
            ..Default::default()
        });
    } 
}

pub fn update_dead_enemies(enemies: &mut Vec<DeadEnemy>, x: &mut f32) {
    let delta = get_frame_time();
    for e in enemies.iter_mut() {
        e.move_tween.update();
        e.opacity_tween.update();
        if e.position.x > *x {
            e.position.x += e.move_tween.value() * delta;
        } else {
            e.position.x -= e.move_tween.value() * delta;
        }
        if e.move_tween.finished() {
            e.active = false;
        }
    }
}

pub fn draw_dead_enemies(texture: Texture2D, enemies: &mut Vec<DeadEnemy>, x: &mut f32, y: &mut f32) {
    for e in enemies.iter() {
        if e.active {
            let mut flip = false;
            if e.position.x > *x {
                flip = true
            }
            draw_texture_ex(
                texture, 
                e.position.x,
                e.position.y,
                Color::new(1.0, 1.0, 1.0, e.opacity_tween.value()),
        DrawTextureParams { 
                    dest_size: Some(vec2(8., 8.)), 
                    source: e.curr_frame,
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

pub struct BatEnemy {
    pub x: f32,
    pub y: f32,
    pub initial_y: f32,
    pub anims: HashMap<String, Animation>,
    pub curr_frame: Option<Rect>
}

impl BatEnemy {
    pub fn new(x: f32, y: f32) -> Self {
        // let tween = Tween::from_keyframes(
        //     vec![
        //         Keyframe::new(0.0, 0.0, EaseOut),
        //         Keyframe::new(20.0, 0.2, EaseOut),
        //     ],
        //     0,
        //     1,
        //     false,
        // );
        let mut idle_state_rects : Vec<Rect> = Vec::new();
        idle_state_rects.push(Rect::new(1., 10., 9., 9.));
        idle_state_rects.push(Rect::new(10., 10., 9., 9.));
    
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

        BatEnemy {
            x,
            y,
            initial_y: y,
            anims,
            curr_frame: Some(Rect::new(1., 10., 9., 9.))
        }
    }
}

pub fn update_bat_enemies_position(enemies: &mut Vec<BatEnemy>) {
    let delta = get_frame_time();
    for e in enemies.iter_mut() {
        e.x += 20. * delta;
        e.y = e.initial_y + ((e.x / 10.)).cos() * 25.;
    }
}

pub fn draw_bat_enemies(texture: Texture2D, enemies: &mut Vec<BatEnemy>) {
    for e in enemies.iter_mut() {
        let frame = e.anims.get_mut("idle").unwrap().get_animation_source(Duration::from_secs_f32(get_frame_time()));
        e.curr_frame = frame;    
        draw_texture_ex(
            texture, 
            e.x,
            e.y,
            WHITE,
    DrawTextureParams { 
                dest_size: Some(vec2(9., 9.)), 
                source: frame,
                // source: Some(Rect::new(1., 10., 9., 9.)),
            ..Default::default()
        });
    }
}

pub fn update_bat_enemies_colliding(
    enemies: &mut Vec<BatEnemy>, 
    x: &f32, y: &f32, hp: &mut f32, 
    player_inv_timer: &mut Timer,
    screen_shake_amount: &mut f32) {
    for e in enemies.iter() {
        let player_pos = Position {
            x: *x,
            y: *y
        };
        let enemy_pos = Position {
            x: e.x,
            y: e.y
        };
        if player_inv_timer.value() == 1.0 {
            if col(player_pos, enemy_pos, 8.) {
                println!("colliding with player");
                // damage_player(hp);
                *screen_shake_amount += 4.0;
                player_inv_timer.restart();
            }
        }
    }
}

pub struct TowerEnemy {
    pub x: f32,
    pub y: f32,
    pub bullet_cooldown: Timer,
    // pub initial_y: f32,
    // pub anims: HashMap<String, Animation>,
    // pub curr_frame: Option<Rect>
}

impl TowerEnemy {
    pub fn new(x: f32, y: f32) -> Self {
        let bullet_cooldown = Timer::new(1000);
        TowerEnemy { x, y, bullet_cooldown }
    }

    fn fire_towards_player(&self, player_x: f32, player_y: f32, bullets: Vec<Bullet>) {

    }

    pub fn update(&mut self, player_x: f32, player_y: f32, bullets: &mut Vec<Bullet>) {
        if self.bullet_cooldown.finished() {
            let mut _dist= 128.;
            let mut _dir: Vec2 = vec2(1.,1.);
            let _d = dist(
                Position { x: self.x, y: self.y} ,
                Position { x: player_x, y: player_y },
            _dist);
            if _d < _dist {
                _dist= _d;
                // _dir = get_dir(e.position.x,e.position.y,*x,*y);
                // let foo = na::Vector2::new(*x, *y);
                // let bar = na::Vector2::new(e.position.x, e.position.y);
                // _dir = foo.sub(bar).norm();
                // println!("{}", _dir);
                _dir = Vec2::new(self.x, self.y) - Vec2::new(player_x, player_y);
                if let Some(d) = _dir.try_normalize() {
                    _dir = d;
                }
            }
            bullets.push(Bullet { x: self.x + 2., y: self.y + 2., dir_x: _dir.x, dir_y: _dir.y, active: true });
            self.bullet_cooldown.restart();         
        }
    }
}

pub fn draw_tower_enemies(texture: Texture2D, enemies: &mut Vec<TowerEnemy>) {
    for e in enemies.iter() {
        draw_rectangle(e.x, e.y, 4., 4., WHITE);
    }
}

pub fn update_tower_enemies(enemies: &mut Vec<TowerEnemy>, player_x: &f32, player_y: &f32, bullets: &mut Vec<Bullet>) {
    for e in enemies.iter_mut() {
        e.update(*player_x, *player_y, bullets);
    }
}

pub fn update_enemy_bullets(bullets: &mut Vec<Bullet>, particles: &mut Vec<Particle>, delta: f32) {
    for bullet in bullets.iter_mut() {
        if bullet.active {
            bullet.x -= bullet.dir_x * delta * 20.; 
            bullet.y -= bullet.dir_y * delta * 20.;
            spawn_particle(particles, bullet.x, bullet.y, Box::new(EnemyShotParticle{}));
        }
    }
}

pub fn bullet_damage_player(
    bullets: &mut Vec<Bullet>, 
    // enemies: &mut Vec<Enemies>,
    x: &f32, y: &f32,
    player_hp: &f32,
    dmg_pop: &mut Vec<DamagePopup>,
    screen_shake_amount: &mut f32,
    // player_dmg: &f32
) {
    for bullet in bullets.iter_mut() {
        // Collide with enemies
        if col(
            Position { x: bullet.x, y: bullet.y }, 
            Position { x: *x + 2., y: *y + 2. }, 
            5.
        ) {
            if *player_hp > 0. {
                bullet.active = false;
                // dmg_pop.push(DamagePopup::new(e.position.x, e.position.y, player_dmg.abs() as i32));
                *screen_shake_amount += 1.0;
                // e.hp -= player_dmg;
                // println!("{}", e.hp);
            }
        }
    }
}

pub fn draw_enemy_bullets(texture: Texture2D, bullets: &mut Vec<Bullet>) {
    for bullet in bullets.iter() {
        if bullet.active {
            draw_texture_ex(
                texture, 
                bullet.x,
                bullet.y, 
                WHITE,
        DrawTextureParams { 
                    dest_size: Some(vec2(8., 8.)), 
                    source: Some(Rect::new(
                        40.,
                        2.,
                        8.,
                        8.,
                    )),
                ..Default::default()
            })
        }
    }
}