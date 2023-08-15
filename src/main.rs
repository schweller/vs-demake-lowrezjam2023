use std::{time::Duration, collections::HashMap, f32::consts::PI};
use keyframe::{Keyframe, functions::EaseOut};
use macroquad::prelude::*;

mod ui;
mod timer;
mod upgrade;
mod tween;
mod enemies;
mod damage_popup;
mod animation;
mod particles;
mod stopwatch;
mod stopwatch_bevy;
use crate::tween::Tween;
use ui::*;
use timer::Timer;
use upgrade::*;
use enemies::*;
use damage_popup::*;
use animation::Animation;
use particles::*;
use stopwatch::*;
use stopwatch_bevy::*;

use ::tween::{Tweener, Oscillator, CircInOut};

// Spawning enemies
// - decide where to spawn ✅
// - spawn ✅

// Taking and dealing damage - done but needs refinement ✅

// Scale difficulty
// - Difficulty tracking ?
// -- System to update difficulty
// -- Difficulty increase spawned monsters qty
// -- Less XP given
// - harder enemies ✅
// - more enemies? ✅

// Level up
// - Change state - ✅
// - Render upgrade choices - ✅
// - Apply upgrades ✅

// Upgrade choices
// - Player Speed ✅
// - Bullet spawn rate ✅
// - HP Recovery rate ✅

// Juicing
// - screen shake ✅
// - flash enemie on hit
// - particles ✅
// - animate sprites ✅
// - sound

// Improve collision
// Collision avoidance?

const PLAYER_SPEED: f32 = 10.;

fn window_conf() -> Conf {
    Conf { 
        window_title: "LowRezJam 2023".to_owned(), 
        window_width: 640, // 640 + 120 
        window_height: 640, // 320 + 120
        window_resizable: false,
        ..Default::default()
    }
}


#[derive(Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

#[derive(Clone, Copy)]
pub struct Collider {
    pub x: f32,
    pub y: f32,
    pub width: i32,
    pub height: i32,
    pub radius: f32
}

fn sprite_rect(ix: u32) -> Rect {
    let sw = 8. as f32;
    let sh = 8. as f32;
    let sx = (ix % 5) as f32 * (sw + 2 as f32) + 2 as f32;
    let sy = (ix / 5) as f32 * (sh + 2 as f32) + 2 as f32;

    // TODO: configure tiles margin
    Rect::new(sx + 1., sy + 1., sw - 2.2, sh - 2.2)
}

fn draw_player(texture: Texture2D, frame: Option<Rect>, x: &mut f32, y: &mut f32, flip_x: &bool, player_inv_timer: &Timer) {
    let mut color = WHITE;
    if player_inv_timer.value() != 1.0 {
        color = Color::new(1.0, 0., 0., 1.);
    }
    draw_texture_ex(
        texture, 
        *x,
        *y, 
        color,
DrawTextureParams { 
            dest_size: Some(vec2(8., 8.)), 
            source: frame,
            flip_x: *flip_x,
        ..Default::default()
    })
}

fn draw_player_collider(x: &mut f32, y: &mut f32) {
    draw_circle(
        *x + 4., 
        *y + 4., 
        4., 
        Color::from_rgba(255, 0, 0, 120)
    );
}

pub fn col(a: Position, b: Position, r: f32) -> bool {
    let x = (b.x - a.x).abs();
    if x > r {
        return false
    }
    let y = (b.y - a.y).abs();
    if y > r {
        return false
    }
    return (x*x+y+y)<r*r
}

pub fn dist(a: Position, b: Position, r: f32) -> f32 {
    let x = (a.x - b.x).abs();
    let y = (a.y - b.y).abs();
    if x+y < r*1.5 {
        let _d = (x*x+y*y).sqrt();
        if _d < r {
            return _d;
        } else {
            return r;
        }
    } else {
        return r;
    }
}

fn get_dir(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    return (x2 - x1).atan2(y2 - y1);
}

pub fn get_dir_(vec1: Position, vec2: Position) -> f32 {
    return (vec2.x - vec1.x).atan2(vec2.y - vec1.y);
}

fn draw_map_cell(texture: Texture2D, x: i32, y: i32) {
    draw_texture_ex(
        texture, 
        x as f32 * 8., 
        y as f32 * 8., 
        WHITE,
DrawTextureParams { 
            dest_size: Some(vec2(8., 8.)), 
            source: Some(Rect { x: 32., y: 2., w: 7., h: 7. }),
        ..Default::default()
    })
}

pub struct Bullet {
    x: f32,
    y: f32,
    dir_x: f32,
    dir_y: f32,
    active: bool
}

fn draw_bullets(texture: Texture2D, bullets: &mut Vec<Bullet>) {
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

fn spawn_bullet(bullets: &mut Vec<Bullet>, enemies: &mut Vec<Enemies>, x: &mut f32, y: &mut f32) {
    let mut _dist= 128.;
    let mut _dir: Vec2 = vec2(1.,1.);
    if enemies.len() > 0 {
        for e in enemies.iter() {
            let _d = dist(
                Position { x: *x, y: *y },
                e.position,
            _dist);
            if _d < _dist {
                _dist= _d;
                // _dir = get_dir(e.position.x,e.position.y,*x,*y);
                // let foo = na::Vector2::new(*x, *y);
                // let bar = na::Vector2::new(e.position.x, e.position.y);
                // _dir = foo.sub(bar).norm();
                // println!("{}", _dir);
                _dir = Vec2::new(*x, *y) - Vec2::new(e.position.x, e.position.y);
                if let Some(d) = _dir.try_normalize() {
                    _dir = d;
                }
            }
        }
        bullets.push(Bullet { x: *x + 2., y: *y + 2., dir_x: _dir.x, dir_y: _dir.y, active: true });
    }
}

fn update_bullets(bullets: &mut Vec<Bullet>, particles: &mut Vec<Particle>) {
    let delta = get_frame_time();
    
    for bullet in bullets.iter_mut() {
        if bullet.active {
            bullet.x -= bullet.dir_x * delta * 20.; 
            bullet.y -= bullet.dir_y * delta * 20.;
            spawn_particle(particles, bullet.x, bullet.y, Box::new(ShotParticle{}));
        }
    }
}

fn damage_enemy(
    bullets: &mut Vec<Bullet>, 
    enemies: &mut Vec<Enemies>, 
    dmg_pop: &mut Vec<DamagePopup>,
    screen_shake_amount: &mut f32,
    player_dmg: &f32
) {
    for e in enemies.iter_mut() {
        for bullet in bullets.iter_mut() {
            // Collide with enemies
            if col(
                Position { x: bullet.x, y: bullet.y }, 
                Position { x: e.position.x + 2., y: e.position.y + 2. }, 
                5.
            ) {
                if e.hp > 0. {
                    bullet.active = false;
                    dmg_pop.push(DamagePopup::new(e.position.x, e.position.y, player_dmg.abs() as i32));
                    *screen_shake_amount += 1.0;
                    e.hp -= player_dmg;
                    // println!("{}", e.hp);
                }
            }
        }
    }
}

fn kill_enemies(enemies: &mut Vec<Enemies>, player_xp: &mut f32, dead_enemies: &mut Vec<DeadEnemy>, kill_count: &mut i32, progression: &mut f32, base_given_xp: &mut f32) {
    for e in enemies.iter_mut() {
        if e.hp <= 0. { 
            e.alive = false;
            *player_xp += e.get_given_xp();
            *kill_count += 1;
            update_progress_level(progression, *kill_count);
            let dead_enemy_obj = DeadEnemy::new(e.position.x, e.position.y, e.curr_frame);
            dead_enemies.push(dead_enemy_obj);
        }
    }
}

fn kill_bat_enemies(enemies: &mut Vec<BatEnemy>, player_xp: &mut f32, dead_enemies: &mut Vec<DeadEnemy>, kill_count: &mut i32, progression: &mut f32, base_given_xp: &mut f32) {
    for e in enemies.iter_mut() {
        if e.hp <= 0. { 
            e.active = false;
            *player_xp += e.given_xp;
            *kill_count += 1;
            update_progress_level(progression, *kill_count);
            let dead_enemy_obj = DeadEnemy::new(e.x, e.y, e.curr_frame);
            dead_enemies.push(dead_enemy_obj);
        }
    }
}

fn level_up_player(player_xp: &mut f32, player_max_xp: &mut f32, mut player_level: &mut i32, state: &mut LevelState) {
    *player_xp = 0.;
    *player_level += 1;
    *state = LevelState::LevelUp;
}

fn level_up_input() -> Option<LevelState> {
    if is_key_pressed(KeyCode::Z) {
        return Some(LevelState::InGame)
    }
    None
}

fn choose_upgrade_input(index: &mut i32, tween: &mut Tween) {
    if is_key_pressed(KeyCode::Right) {
        if *index == 1 {
            return;
        } else {
            *index += 1;
            tween.restart();
        }
    }
    if is_key_pressed(KeyCode::Left) {
        if *index == 0 {
            return;
        } else {
            *index -= 1;
            tween.restart();
        }
    }
}

pub fn axis(negative: bool, positive: bool) -> f32 {
    ((positive as i8) - (negative as i8)) as f32
}

fn move_player(x: &mut f32, y: &mut f32, flip_x: &mut bool, speed: &f32, delta: f32) {
    let a = axis(is_key_down(KeyCode::Left), is_key_down(KeyCode::Right));
    let b = axis(is_key_down(KeyCode::Down), is_key_down(KeyCode::Up));
    let magnitude = (a.powi(2) + b.powi(2)).sqrt();
    let foo_x = a / magnitude;
    let foo_y = b / magnitude;
    if a != 0. || b != 0. {
        *x += foo_x * delta * PLAYER_SPEED * speed;
        *y -= foo_y * delta * PLAYER_SPEED * speed;
    }

    if a == -1. { *flip_x = true }
    if a == 1. { *flip_x = false }

    // if is_key_down(KeyCode::Left) {
    //     *x -= (PLAYER_SPEED * speed) * delta;
    //     *flip_x = true;
    // }
    // if is_key_down(KeyCode::Right) {
    //     *x += (PLAYER_SPEED * speed) * delta;
    //     *flip_x = false;
    // }
    // if is_key_down(KeyCode::Up) {
    //     *y -= (PLAYER_SPEED * speed) * delta;
    // }
    // if is_key_down(KeyCode::Down) {
    //     *y += (PLAYER_SPEED * speed) * delta;
    // }
}

enum LevelState {
    PreGame,
    LevelUp,
    InGame,
    StageCleared
}

fn get_minutes_from_millis(elapsed_time: u128) -> String {
    let mins = (elapsed_time/1000)/60;
    if mins < 10 {
        return "0".to_string() + &mins.to_string();
    } else {
        mins.to_string()
    }
}

fn get_seconds_from_millis(elapsed_time: u128) -> String {
    let secs = (elapsed_time/1000)%60;
    if secs < 10 {
        return "0".to_string() + &secs.to_string();
    } else {
        secs.to_string()
    }
}

pub type TestTween<Value, Time> = Tweener<Value, Time, Box<dyn ::tween::Tween<Value>>>;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up, Down, Left, Right, UpLeft, UpRight, DownLeft, DownRight,
}

pub fn get_direction() -> Option<Direction> {
    let mut dir_vec = vec2(0.,0.);
    
    if is_key_down(KeyCode::Up) {
        dir_vec.y = -1.;
    } 
    if is_key_down(KeyCode::Down) {
        dir_vec.y = 1.; 
    }
     if is_key_down(KeyCode::Left) {
        dir_vec.x = -1.;
    }
    if is_key_down(KeyCode::Right) {
        dir_vec.x = 1.;
    }

    if dir_vec.x == 0. && dir_vec.y == -1. {
        Some(Direction::Up)
    } else if dir_vec.x == 0. && dir_vec.y == 1. {
        Some(Direction::Down)
    } else if dir_vec.x == -1. && dir_vec.y == 0. {
        Some(Direction::Left)
    } else if dir_vec.x == 1. && dir_vec.y == 0. {
        Some(Direction::Right)
    } else if dir_vec.x == 1. && dir_vec.y == 1. {
        Some(Direction::DownRight)
    } else if dir_vec.x == -1. && dir_vec.y == -1. {
        Some(Direction::UpLeft)
    } else if dir_vec.x == 1. && dir_vec.y == -1. {
        Some(Direction::UpRight)
    } else if dir_vec.x == -1. && dir_vec.y == 1. {
        Some(Direction::DownLeft)
    } else {
        None
    }
}

pub fn update_progress_level(progression: &mut f32, kill_count: i32) {
    // println!(" rem {}", kill_count.rem_euclid(15));
     if kill_count > 0 && kill_count.rem_euclid(15) == 0 {
        *progression += 1.0;
     }
}

pub fn update_given_xp(base_given_xp: &mut f32, kill_count: i32) {
    let b = *base_given_xp;
    *base_given_xp -= (0.1 * (b)) - (0.2 * (kill_count as f32))
}

#[macroquad::main(window_conf)]
async fn main() {
    let camera_zoom : f32 = 10.0;

    // This absolute mess which I will change one day
    let main_texture = load_texture("assets/vs-dx-atlas-padded.png").await.unwrap();
    main_texture.set_filter(FilterMode::Nearest);
    let ui_texture = load_texture("assets/vs-dx-ui-atlas.png").await.unwrap();
    ui_texture.set_filter(FilterMode::Nearest);
    let upgrade_texture = load_texture("assets/vs-dx-upgrades-atlas.png").await.unwrap();
    upgrade_texture.set_filter(FilterMode::Nearest);
    let font = load_ttf_font("assets/smolFontMono.ttf").await.unwrap();
    font.set_filter(FilterMode::Nearest);
    let player_texture = load_texture("assets/vs-dx-player-atlas.png").await.unwrap();
    player_texture.set_filter(FilterMode::Nearest);
    let slime_texture = load_texture("assets/vs-dx-enemies-atlas.png").await.unwrap();
    slime_texture.set_filter(FilterMode::Nearest);
    let main_title_texture = load_texture("assets/vs-dx-maintitle-atlas.png").await.unwrap();
    main_title_texture.set_filter(FilterMode::Nearest);

    // Player definitions
    let mut player_pos_x = 128.;
    let mut player_pos_y = 128.;
    let player_max_hp = 100.;
    let mut player_hp : f32 = player_max_hp;
    let mut player_max_xp = 100.;
    let mut player_xp = 0.;
    let mut player_level = 1;
    let mut current_player_hp_percentage; 
    let mut current_player_xp_percentage;
    let mut player_flip_x: bool = false;
    let mut player_speed_bonus = 1.;
    let mut player_regen = 1.;
    let mut player_regen_timer = Timer::new(5000);
    let mut player_inv_timer = Timer::new(1800);
    let mut player_is_dashing = false;
    let mut player_direction = None;
    let mut dashing_timer = Timer::new(500);
    let mut dash_speed = 40.0;
    let mut player_active = true;
    let player_damage = 2.;

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

    // Level up UI definitions
    let mut choosen_upgrade_index = 0;

    // Enemies & Bullets & Particles
    let mut enemies: Vec<Enemies> = Vec::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut dead_enemies: Vec<DeadEnemy> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    let mut intro_particles: Vec<Particle> = Vec::new();
    let max_b_cooldown = Timer::new(700);
    let mut bullet_cooldown = max_b_cooldown;
    let mut current_bullet_cooldown_bonus = 1.0;
    let max_enemy_cooldown = Duration::from_secs(8).as_millis();
    let mut enemy_cooldown = max_enemy_cooldown;
    let mut enemy_bullets: Vec<Bullet> = Vec::new();

    // let mut upgrades: Vec<Box<dyn Upgrade>> = Vec::new();
    let mut upgrades: Vec<Box<dyn Upgrade>> = pick_random_upgrades();

    let mut bat_enemies : Vec<BatEnemy> = Vec::new();
    let mut tower_enemies: Vec<TowerEnemy> = Vec::new();

    // Level Transition tweener
    // & Tweeners in general
    // Tween for upgrade
    let mut upgrade_menu_tween = Tween::from_keyframes(
        vec![
            Keyframe::new(0.0, 0.0, EaseOut),
            Keyframe::new(4.0, 0.5, EaseOut),
            Keyframe::new(0.0, 1.0, EaseOut),
        ],
        0,
        1,
        true,
    );    
    let mut tweener = Tweener::sine_in_out(0., screen_width(), 5.0);
    let mut test_tweener : TestTween<f32, f32> = Tweener::new(0., 10., 1.5, Box::new(Oscillator::new(CircInOut)));
    let mut level_up_letters_tweener : TestTween<f32, f32> = Tweener::new(
        -3., 
        3., 
        2.2, 
        Box::new(Oscillator::new(::tween::SineInOut))
    );
    let mut main_title_tweener : TestTween<f32, f32> = Tweener::new(
        -10., 
        10., 
        2.2, 
        Box::new(Oscillator::new(::tween::SineInOut))
    );    
    let mut init_upgrade_tweener : TestTween<f32, f32> = Tweener::new(0., 10., 1.5, Box::new(CircInOut));
    let mut death_tweener = Tweener::sine_in_out(0., 10., 2.);
    
    // let mut sw = StopWatch::start();
    let mut sw = Stopwatch::new();
    
    let mut damage_popups : Vec<DamagePopup> = Vec::new();
    let mut screen_shake_amount: f32 = 0.;

    // Progression
    // 1 - xp 40 
    // 2 - xp 36 -> base - (2*(base*5%)) - (killcount*10%)
    // 3 - xp 30.4 -> 
    // 4 - xp 23
    let mut progression = 1.0;
    let mut base_given_xp: f32 = 50.0;
    let mut kill_count = 0;

    let mut level_state = LevelState::PreGame;
    loop {
        clear_background(Color::from_rgba(37, 33, 41, 255));
        let delta = get_frame_time();

        // still not sure here
        // request_new_screen_size(640., 640.);
        screen_shake_amount *= 0.94;

        let screen_shake = Vec2::new(
            rand::gen_range(-screen_shake_amount, screen_shake_amount),
            rand::gen_range(-screen_shake_amount, screen_shake_amount),
        );
        let camera_focal_y = player_pos_y;
        let camera_focal_x = player_pos_x;
        set_camera(&Camera2D {
            target: vec2(lerp(camera_focal_x + 4., camera_focal_x - 4., get_frame_time()), lerp(camera_focal_y + 4., camera_focal_y - 4., get_frame_time())) + screen_shake,
            zoom: Vec2::new(
                camera_zoom / 640. * 2., 
                -camera_zoom / 640. * 2.
            ),
            ..Default::default()
        });

        match level_state {
            LevelState::PreGame => {
                clear_background(Color::from_hex(0x252129));
                set_default_camera();

                draw_particles(&mut intro_particles);
                draw_texture_ex(main_title_texture, screen_width()/2.-180., 100. + main_title_tweener.move_by(delta), WHITE, 
                    DrawTextureParams { 
                        dest_size: Some(vec2(36. * 10., 22. * 10.)), 
                        source: Some(Rect::new(5., 3., 36., 22.)),
                        ..Default::default()
                    }
                );
                draw_texture_ex(main_title_texture, screen_width()/2.-290., screen_height() - 300., WHITE, 
                    DrawTextureParams { 
                        dest_size: Some(vec2(58. * 10., 18. * 10.)), 
                        source: Some(Rect::new(5., 30., 58., 18.)),
                        ..Default::default()
                    }
                );
                spawn_particle(
                    &mut intro_particles, 
                    screen_width()/4., 
                    0.,
                    Box::new(IntroParticle{})
                );
                update_particles(&mut intro_particles);
                                
                if is_key_pressed(KeyCode::Z) {
                    // restart the "game state"
                    sw = Stopwatch::new();
                    player_pos_x = 128.;
                    player_pos_y = 128.;
                    player_hp = player_max_hp;
                    player_max_xp = 100.;
                    player_xp = 0.;
                    player_level = 1;
                    player_flip_x = false;
                    player_speed_bonus = 1.;
                    player_regen = 1.;
                    player_regen_timer = Timer::new(5000);
                    player_inv_timer = Timer::new(1800);
                    player_is_dashing = false;
                    player_direction = None;
                    dashing_timer = Timer::new(500);
                    dash_speed = 40.0;
                    player_active = true;

                    upgrade_menu_tween = Tween::from_keyframes(
                        vec![
                            Keyframe::new(0.0, 0.0, EaseOut),
                            Keyframe::new(4.0, 0.5, EaseOut),
                            Keyframe::new(0.0, 1.0, EaseOut),
                        ],
                        0,
                        1,
                        true,
                    );    
                    tweener = Tweener::sine_in_out(0., screen_width(), 5.0);
                    test_tweener = Tweener::new(0., 10., 1.5, Box::new(Oscillator::new(CircInOut)));
                    level_up_letters_tweener = Tweener::new(
                        -3., 
                        3., 
                        2.2, 
                        Box::new(Oscillator::new(::tween::SineInOut))
                    );
                    init_upgrade_tweener = Tweener::new(0., 10., 1.5, Box::new(CircInOut));
                    death_tweener = Tweener::sine_in_out(0., 10., 2.);                    

                    level_state = LevelState::InGame;
                }
                // tween to start
            }
            LevelState::InGame => {
                sw.tick(Duration::from_secs_f32(0.01));
                choosen_upgrade_index = 0;
                for x in 0..160 {
                    for y in 0..100 {
                        draw_map_cell(main_texture, x, y);
                    }
                }
                
                // Update block
                if player_active {
                    // Move and Dashing input block
                    if is_key_pressed(KeyCode::X) && !player_is_dashing {
                        player_direction = get_direction();
                        if let Some(_dir) = player_direction {
                            dashing_timer.restart();
                            player_is_dashing = true;
                        }
                    }
                    if !player_is_dashing {
                        move_player(
                            &mut player_pos_x, 
                            &mut player_pos_y, 
                            &mut player_flip_x, 
                            &player_speed_bonus, 
                            delta
                        );
                    }
                    update_enemies_position(&mut enemies, &mut player_pos_x, &mut player_pos_y);
                    update_enemies_pushing(&mut enemies);
                    update_enemies_colliding(&mut enemies, &mut player_pos_x, &mut player_pos_y, &mut player_hp, &player_is_dashing, &mut player_inv_timer, &mut screen_shake_amount);
                    update_bat_enemies_position(&mut bat_enemies);
                    update_bat_enemies_colliding(
                        &mut bat_enemies, 
                        &mut player_pos_x, 
                        &mut player_pos_y, 
                        &mut player_hp, 
                        &mut player_inv_timer, 
                        &mut screen_shake_amount, 
                        &mut damage_popups,
                        &player_is_dashing
                    );
                    update_tower_enemies(&mut tower_enemies, &player_pos_x, &player_pos_y, &mut enemy_bullets);
                    update_bullets(&mut bullets, &mut particles);
                    update_enemy_bullets(&mut enemy_bullets, &mut particles, delta);
                }
                update_dead_enemies(&mut dead_enemies, &mut player_pos_x);
                update_particles(&mut particles);
                
                if player_is_dashing {        
                    // Calculate the dash distance based on the dash speed and delta time
                    let dash_distance = dash_speed * delta;
        
                    // Update player position based on dash direction
                    // 0.7071 was pure experimentation
                    match player_direction {
                        Some(Direction::Up) => player_pos_y -= dash_distance,
                        Some(Direction::Down) => player_pos_y += dash_distance,
                        Some(Direction::Left) => player_pos_x -= dash_distance,
                        Some(Direction::Right) => player_pos_x += dash_distance,
                        Some(Direction::UpLeft) => {
                            player_pos_x -= dash_distance * 0.7071;
                            player_pos_y -= dash_distance * 0.7071;
                        }
                        Some(Direction::UpRight) => {
                            player_pos_x += dash_distance * 0.7071;
                            player_pos_y -= dash_distance * 0.7071;
                        }
                        Some(Direction::DownLeft) => {
                            player_pos_x -= dash_distance * 0.7071;
                            player_pos_y += dash_distance * 0.7071;
                        }
                        Some(Direction::DownRight) => {
                            player_pos_x += dash_distance * 0.7071;
                            player_pos_y += dash_distance * 0.7071;
                        }
                        None => {}
                    }

                    spawn_particle(
                        &mut particles, 
                        player_pos_x, 
                        player_pos_y,
                        Box::new(PlayerDashParticle{ texture: player_texture})
                    );

                    if dashing_timer.finished() {
                        player_is_dashing = false;
                    }
                }

                let player_frame = anims.get_mut("idle").unwrap().get_animation_source(Duration::from_secs_f32(get_frame_time()));
                
                // Draw block
                draw_particles(&mut particles);
                // player.draw(player_texture, frame);
                draw_player(
                    player_texture,
                    player_frame, 
                    &mut player_pos_x, 
                    &mut player_pos_y, 
                    &player_flip_x, 
                    &player_inv_timer
                );
                draw_enemies(
                    slime_texture, 
                    &mut enemies, 
                    &mut player_pos_x, 
                    &mut player_pos_y
                );
                draw_tower_enemies(slime_texture, &mut tower_enemies);
                draw_bat_enemies(
                    slime_texture,
                    &mut bat_enemies
                );
                draw_dead_enemies(slime_texture, &mut dead_enemies, &mut player_pos_x, &mut player_pos_y);
                draw_bullets(main_texture, &mut bullets);
                draw_enemy_bullets(main_texture, &mut enemy_bullets);

                // draw_player_collider(&mut player_pos_x, &mut player_pos_y);
                // draw_enemies_collider(&mut enemies);

                for popup in damage_popups.iter_mut() {
                    popup.update();
                    popup.draw(font);
                }

                // Spawning enemies
                // Count slimes
                if player_active {
                    if enemies.len() < (5*(progression as usize)) {
                        let mut given_xp = base_given_xp - (0.1 * (base_given_xp)) - (0.5 * (kill_count as f32)) - progression*4.;
                        if given_xp < 3. { given_xp = 3. }
                        // println!("{} given_xp", given_xp);                    
                        spawn_enemies(&mut enemies, &player_pos_x, &player_pos_y, given_xp);
                    }

                    // Count Bats
                    if bat_enemies.len() < (2*(progression as usize)) {
                        let x_dir: f32;
                        match rand::gen_range(0, 2) {
                            0 => x_dir = -1.,
                            _ => x_dir = 1.,
                        }
                        let spawn_pos_y = player_pos_y * (rand::gen_range(0.5, 2.));
                        let mut given_xp = base_given_xp - (0.1 * (base_given_xp)) - (0.5 * (kill_count as f32)) - progression*4.;
                        if given_xp < 3. { given_xp = 3. }
                        println!("{} given_xp", given_xp);
                        bat_enemies.push(
                            BatEnemy::new(player_pos_x - 64. * (x_dir), spawn_pos_y, x_dir, given_xp)
                        );
                    }
                    // And towers
                    if progression >= 3. {
                        if tower_enemies.len() < (2*progression as usize) {
                            // Random around radius
                            // let angle = rand::gen_range(0.0, std::f32::consts::TAU); // Random angle in radians
                            // let distance = rand::gen_range(20.,100.); // Random distance within the spawn radius
                        
                            // let spawn_x = player_pos_x + distance * angle.cos();
                            // let spawn_y = player_pos_y + distance * angle.sin();
                        
                            // Random around circ 
                            let angle = rand::gen_range(0.0,std::f32::consts::TAU); // Random angle in radians
        
                            let spawn_x = player_pos_x + 32. * angle.cos();
                            let spawn_y = player_pos_y + 32. * angle.sin();
        
                            tower_enemies.push(
                                TowerEnemy::new(spawn_x, spawn_y)
                            );
                        }
                    }
                }

                if player_regen_timer.finished() && player_active {
                    if player_hp + player_regen >= player_max_hp {
                        player_hp = player_max_hp
                    } else {
                        player_hp += player_regen;
                    }
                    player_regen_timer.restart();
                }
 
                damage_enemy(&mut bullets, &mut enemies, &mut damage_popups, &mut screen_shake_amount, &player_damage);
                bullet_damage_player(&mut enemy_bullets, &player_pos_x, &player_pos_y, &mut player_hp, &mut damage_popups, &mut screen_shake_amount, &mut player_inv_timer, &player_is_dashing);
                kill_enemies(&mut enemies, &mut player_xp, &mut dead_enemies, &mut kill_count, &mut progression, &mut base_given_xp);
                kill_bat_enemies(&mut bat_enemies, &mut player_xp, &mut dead_enemies, &mut kill_count, &mut progression, &mut base_given_xp);
                clean_bat_enemies(&mut bat_enemies, &player_pos_x, &player_pos_y);

                if player_xp >= player_max_xp {
                    upgrades = pick_random_upgrades();
                    level_up_player(&mut player_xp, &mut player_max_xp, &mut player_level, &mut level_state);
                }
                
                if bullet_cooldown.finished() {
                    spawn_bullet(&mut bullets, &mut enemies, &mut player_pos_x, &mut player_pos_y);
                    bullet_cooldown.set_duration_millis(((3000 as f32) * current_bullet_cooldown_bonus) as u64);
                    bullet_cooldown.restart();
                }

                // Get rid of things that shouldn't be around anymore
                // Bullets, enemies, particles, pop-ups
                bullets.retain(|b| b.active);
                enemy_bullets.retain(|b| b.active);
                enemies.retain(|e| e.alive);
                bat_enemies.retain(|e| e.active);
                tower_enemies.retain(|e| e.active);
                dead_enemies.retain(|e| e.active);
                damage_popups.retain(|e| e.active);
                particles.retain(|p| p.active);        

                set_default_camera();

                current_player_hp_percentage = (player_hp / player_max_hp) * 100.;
                current_player_xp_percentage = (player_xp / player_max_xp) * 100.;
                draw_level_ui(ui_texture, &current_player_hp_percentage, &current_player_xp_percentage, &player_level, &player_inv_timer);
                draw_level_timer_ui(
                    font, 
                    get_minutes_from_millis(sw.elapsed().as_millis()), 
                    get_seconds_from_millis(sw.elapsed().as_millis())
                );

                if player_hp <= 0. {
                    death_tweener.move_by(delta);
                    player_hp = 0.;
                    player_active = false;
                    sw.pause();

                    enemies = Vec::new();
                    bat_enemies = Vec::new();
                    bullets = Vec::new();
                    dead_enemies = Vec::new();
                    damage_popups = Vec::new();
                    particles = Vec::new();

                    player_pos_x = -999.;
                    player_pos_y = -999.;


                    screen_shake_amount += 0.5 * 1.1;

                    if death_tweener.is_finished() {
                        level_state = LevelState::PreGame;
                    }
                }

                // Trigger end game progression
                if sw.elapsed().as_millis() > 240000 && player_active {
                    // destroy all entities 
                    // but the player
                    // - deallocates but not sure if its good
                    enemies = Vec::new();
                    bat_enemies = Vec::new();
                    bullets = Vec::new();
                    dead_enemies = Vec::new();
                    damage_popups = Vec::new();
                    particles = Vec::new();

                    if sw.elapsed().as_millis() < 246000 {
                        screen_shake_amount += 0.5;
                    }

                    if sw.elapsed().as_millis() > 248000 {
                        draw_rectangle(
                            0., 
                            0., 
                            tweener.move_by(delta), 
                            screen_height(), 
                            Color::from_rgba(37, 33, 41, 255)
                        );
                        if tweener.is_finished() {
                            level_state = LevelState::StageCleared
                        }
                    }
                }
            },
            LevelState::StageCleared => {
                clear_background(Color::from_rgba(37, 33, 41, 255));
                // Thank player
                // show kill count
                // credits
                // press Z to return to PreGame

                set_default_camera();
                draw_text_ex(
                    "You survived!",
                    (screen_width() / 2.) - 200., 
                    150., 
                    TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
                );
            
                draw_text_ex(
                    "Thanks for playing!",
                    (screen_width() / 2.) - 300., 
                    250., 
                    TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
                );

                draw_text_ex(
                    "Made by inacho",
                    (screen_width() / 2.) - 230., 
                    350., 
                    TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
                );

                draw_text_ex(
                    "For LowRezJam2023",
                    (screen_width() / 2.) - 280., 
                    400., 
                    TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
                );                

                draw_text_ex(
                    "Press Z to restart",
                    (screen_width() / 2.) - 300., 
                    500., 
                    TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
                );  

                if is_key_pressed(KeyCode::Z) {
                    level_state = LevelState::PreGame;
                }
            }
            LevelState::LevelUp => {
                for x in 0..80 {
                    for y in 0..50 {
                        draw_map_cell(main_texture, x, y);
                    }
                }
        
                sw.pause();
                let frame = anims.get_mut("idle").unwrap().get_animation_source(Duration::from_secs_f32(get_frame_time()));
                draw_player(player_texture, frame, &mut player_pos_x, &mut player_pos_y, &player_flip_x, &player_inv_timer);
                // draw_player_collider(&mut player_pos_x, &mut player_pos_y);
                draw_enemies(slime_texture, &mut enemies, &mut player_pos_x, &mut player_pos_y);
                draw_enemies_collider(&mut enemies);
                draw_bullets(main_texture, &mut bullets);

                // In-level UI
                draw_rectangle(0., screen_height() - 80., screen_width(), 120., BLACK);
                set_default_camera();

                choose_upgrade_input(&mut choosen_upgrade_index, &mut upgrade_menu_tween);
                let result = level_up_input();
                if let Some(newstate) = result {
                    // fine tune!
                    let idx = choosen_upgrade_index as usize;
                    let upg = upgrades[idx].get_name();
                    match upg {
                        "Speed" => {
                            // println!("Speed upgrade");
                            player_speed_bonus += 0.1;
                        }
                        "FireRate" => {
                            current_bullet_cooldown_bonus -= 0.1;
                            // println!("FireRate upgrade");
                        }
                        "Recovery" => {
                            player_regen += 2.;
                            // println!("Recovery upgrade");
                        }
                        "FasterRecovery" => {
                            let dur = player_regen_timer.duration;
                            let dur_5_percent = (player_regen_timer.duration.as_millis() as f32) * 0.05;
                            // println!("recovery {} new recovery {}, 5percent {}", dur.as_millis(), (dur.as_millis() - (dur_5_percent.round() as u128)), dur_5_percent);
                            player_regen_timer.set_duration_millis((dur.as_millis() - (dur_5_percent.round() as u128)) as u64);
                        }
                        "Dash" => {
                            let dash_5_percent = dash_speed * 0.05;
                            dash_speed += dash_5_percent;
                            // println!("{}", dash_speed);
                        },
                        "MoreIframes" => {
                            let dur = player_inv_timer.duration;
                            let extra = (player_inv_timer.duration.as_millis() as f32) * 0.1;
                            // println!("recovery {} new recovery {}, 5percent {}", dur.as_millis(), (dur.as_millis() + (extra.round() as u128)), extra);
                            player_inv_timer.set_duration_millis((dur.as_millis() + (extra.round() as u128)) as u64);
                        }
                        _ => {}
                    }
                    sw.unpause();
                    level_state = newstate;
                }

                current_player_hp_percentage = (player_hp / player_max_hp) * 100.;
                current_player_xp_percentage = (player_xp / player_max_xp) * 100.;
                draw_level_ui(ui_texture, &current_player_hp_percentage, &current_player_xp_percentage, &player_level, &player_inv_timer);
                draw_level_up(
                    &choosen_upgrade_index, 
                    &upgrades, 
                    font, 
                    &mut upgrade_menu_tween, 
                    &mut init_upgrade_tweener
                );
                draw_level_up_title(font, &mut test_tweener, &mut level_up_letters_tweener);
            }
        }

        next_frame().await;
    }
}