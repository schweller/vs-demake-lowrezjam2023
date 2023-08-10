use std::{time::Duration, ops::{Sub, Add}, cmp::min};
use keyframe::{Keyframe, functions::{EaseOut, EaseInOut, EaseIn}};
use macroquad::prelude::*;

mod ui;
mod timer;
mod upgrade;
mod tween;
mod enemies;
mod damage_popup;
use crate::tween::Tween;
use ui::*;
use timer::Timer;
use upgrade::*;
use enemies::*;
use damage_popup::*;

use ::tween::{Tweener, TweenValue, Tween as OtherTween, TweenTime, Looper, Oscillator};

// Spawning enemies
// - decide where to spawn
// - spawn 
// - fix spawning to compensate for bottom UI - prob not being able to done

// Taking and dealing damage - done but needs refinement

// Scale difficulty
// - harder to level up
// - harder enemies
// - more enemies?

// Level up
// - Change state - done
// - Render upgrade choices - done

// Juicing
// - screen shake
// - flash enemie on hit
// - particles
// - animate sprites
// - sound

// Improve collision
// Collision avoidance?

// Upgrade choices
// - Player Speed
// - Bullet spawn rate
// - HP Recovery rate
// - Companion (1-off)

const PLAYER_SPEED: f32 = 10.;

fn window_conf() -> Conf {
    Conf { 
        window_title: "LowRezJam 2023".to_owned(), 
        window_width: 640, // 640 + 120 
        window_height: 640, // 320 + 120
        high_dpi: false,
        window_resizable: false,
        sample_count: 10,
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

fn draw_player(texture: Texture2D, x: &mut f32, y: &mut f32, flip_x: &bool, player_inv_timer: &Timer) {
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
            source: Some(Rect::new(
                10.,
                2.,
                8.,
                8.,
            )),
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

fn update_bullets(bullets: &mut Vec<Bullet>, enemies: &mut Vec<Enemies>) {
    let delta = get_frame_time();
    for bullet in bullets.iter_mut() {
        if bullet.active {
            // let bull_vec = Vec2::new(bullet.x, bullet.y);
            bullet.x -= bullet.dir_x * delta * 20.; 
            bullet.y -= bullet.dir_y * delta * 20.;
        }
    }
}

fn damage_enemy(bullets: &mut Vec<Bullet>, enemies: &mut Vec<Enemies>, player_xp: &mut f32, dmg_pop: &mut Vec<DamagePopup>) {
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
                    dmg_pop.push(DamagePopup::new(e.position.x, e.position.y));
                    e.hp -= 1.;
                }
            }
        }
    }
}

fn kill_enemies(enemies: &mut Vec<Enemies>, player_xp: &mut f32, dead_enemies: &mut Vec<Enemies>) {
    for e in enemies.iter_mut() {
        if e.hp <= 0. { 
            e.alive = false;
            *player_xp += 40.;
            dead_enemies.push(Enemies::new(e.position.x, e.position.y));
        }
    }
}

fn level_up_player(player_xp: &mut f32, player_max_xp: &mut f32, mut player_level: &mut i32, state: &mut LevelState) {
    *player_xp = 0.;
    *player_level += 1;
    *state = LevelState::LevelUp;
}

fn level_up_input(state: &mut LevelState) -> Option<LevelState> {
    if is_key_pressed(KeyCode::Space) {
        return Some(LevelState::InGame)
    }
    None
}

fn choose_upgrade_input(index: &mut i32, tween: &mut Tween) {
    if is_key_pressed(KeyCode::Right) {
        if *index == 2 {
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

fn move_player(x: &mut f32, y: &mut f32, flip_x: &mut bool, speed: &f32, is_dashing: &bool) {
    if !is_dashing {
        let delta = get_frame_time();
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
    }

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

// fn dash_player(x: &mut f32, y: &mut f32, is_dashing: &bool) {
//     if *is_dashing {
//         let delta = get_frame_time();
//         let a = axis(is_key_down(KeyCode::Left), is_key_down(KeyCode::Right));
//         let b = axis(is_key_down(KeyCode::Down), is_key_down(KeyCode::Up));
//         let magnitude = (a.powi(2) + b.powi(2)).sqrt();
//         let foo_x = a / magnitude;
//         let foo_y = b / magnitude;
//         if a != 0. || b != 0. {
//             *x += a * delta * PLAYER_SPEED + (*x * 0.125 * delta)*3.;
//             *y -= b * delta * PLAYER_SPEED + (*y * 0.125 * delta)*3.;
//         }
//         // *x += delta * PLAYER_SPEED + (*x * 0.125 * delta)*3.;
//     }
// }

enum LevelState {
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

/// These are our two control points
pub struct CubicBezier<T>(T, T);

/// This is going to be a strictly speaking **better* implementation:
/// we're going to implement our Tween generically here. That means, although we'll
/// only use Points in this example, you could use this to cubic bezier tween anything.
fn cubic_bezier_for_real(
    start: Point,
    destination: Point,
    duration: f32,
    quarter_pt: Point,
    three_quarter_pt: Point,
) -> Tweener<Point, f32, CubicBezier<Point>> {
    impl<T: TweenValue> OtherTween<T> for CubicBezier<T> {
        fn tween(&mut self, delta: T, t: f32) -> T {
            // we need to write our own lerp with the generic functions available to us
            fn lerp<T: TweenValue>(a: T, b: T, t: f32) -> T {
                (b - a).scale(t) + a
            }

            // cheeky way to get a zero
            let zero = delta.scale(0.0);

            let a = lerp(zero, self.0, t);
            let b = lerp(self.0, self.1, t);
            let c = lerp(self.1, delta, t);

            let d = lerp(a, b, t);
            let e = lerp(b, c, t);

            lerp(d, e, t)
        }

        // oh yeah, we're wild
        fn is_finite(&self) -> bool {
            false
        }
    }

    Tweener::new(start, destination, duration, CubicBezier(quarter_pt, three_quarter_pt))
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point(f32, f32);

impl Point {
    /// Moves us towards the other Point by a factor of `t`
    fn lerp(self, other: Self, t: f32) -> Self {
        self.scale(1.0 - t) + other.scale(t)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl TweenValue for Point {
    fn scale(self, scale: f32) -> Self {
        Self(self.0 * scale, self.1 * scale)
    }
}

// pub fn draw_bouncy_text(font: Font, text: String, x: f32, y: f32) {
//     let t = get_frame_time();
//     let n = min((t as usize) * 50,text.chars().count());
//     // const t: f32 = 1.0 / 60.0;
//     let mut pos_x = x - (text.chars().count() as f32) * 2.;

//     let params = TextParams {
//         font,
//         font_size: 64,
//         ..Default::default()
//     };

//     // println!("{}", text.chars().count());
//     for i in 0..text.chars().count() {
//         let substr = &text[i..i+1];
//         println!("{}", substr);
//         let xx = pos_x;
//         let yy = y + 70. * ((i as f32 / 10.) + t * 10. ).cos() - ((i as f32 / 5.) - t * 1.7 + 20.).cos();
//         draw_text_ex(
//             substr, 
//             xx,
//             yy - 1.,
//             TextParams {
//                 font,
//                 font_size: 64,
//                 ..Default::default()
//             }
//         );
//         draw_text_ex(
//             substr, 
//             xx - 1.,
//             yy,
//             TextParams {
//                 font,
//                 font_size: 64,
//                 ..Default::default()
//             }
//         );
//         draw_text_ex(
//             substr, 
//             xx + 1.,
//             yy,
//             TextParams {
//                 font,
//                 font_size: 64,
//                 ..Default::default()
//             }
//         );
//         draw_text_ex(
//             substr, 
//             xx - 1.,
//             yy + 1.,
//             TextParams {
//                 font,
//                 font_size: 64,
//                 ..Default::default()
//             }
//         );  
//         draw_text_ex(
//             substr, 
//             xx + 1.,
//             yy + 1.,
//             params
//         );
//         draw_text_ex(
//             substr, 
//             xx,
//             yy + 2.,
//             params
//         );
//         draw_text_ex(
//             substr, 
//             xx,
//             yy + 1.,
//             params
//         );
//         draw_text_ex(
//             substr, 
//             xx,
//             yy,
//             params
//         );        
//         pos_x += 30.;    
//     }
// }

// fn to_f32(self) -> f32 {
//     self as f32
// }

#[macroquad::main(window_conf)]
async fn main() {
    let min_camera_zoom = 1.3;
    let max_camera_zoom = 2.0;
    let mut camera_focal_y = screen_height() / 2.0;
    let mut camera_focal_x = screen_width() / 2.0;
    let main_area_width = 570.;
    let camera_zoom : f32 = 10.0;

    let main_texture = load_texture("assets/vs-dx-atlas-padded.png").await.unwrap();
    main_texture.set_filter(FilterMode::Nearest);
    let ui_texture = load_texture("assets/vs-dx-ui-atlas.png").await.unwrap();
    ui_texture.set_filter(FilterMode::Nearest);
    let upgrade_texture = load_texture("assets/vs-dx-upgrades-atlas.png").await.unwrap();
    upgrade_texture.set_filter(FilterMode::Nearest);
    let font = load_ttf_font("assets/smolFontMono.ttf").await.unwrap();

    // Player definitions
    let mut player_pos_x = 64.;
    let mut player_pos_y = 64.;
    let player_max_hp = 100.;
    let mut player_hp : f32 = player_max_hp;
    let mut current_player_hp_percentage; 
    let mut player_max_xp = 100.;
    let mut player_xp = 1.;
    let mut player_level = 1;
    let mut current_player_xp_percentage;
    let mut player_flip_x: bool = false;
    let mut player_speed_bonus = 1.;
    let mut player_inv_timer = Timer::new(1800);

    // Level up UI definitions
    let mut choosen_upgrade_index = 0;

    // Enemies & Bullets
    let mut enemies: Vec<Enemies> = Vec::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut dead_enemies: Vec<Enemies> = Vec::new();
    let max_cooldown = Duration::from_secs(15).as_millis();
    let mut bullet_cooldown = max_cooldown;

    let max_enemy_cooldown = Duration::from_secs(8).as_millis();
    let mut enemy_cooldown = max_enemy_cooldown;

    let mut level_state = LevelState::LevelUp;

    // let mut upgrades: Vec<Box<dyn Upgrade>> = Vec::new();
    let mut upgrades: Vec<Box<dyn Upgrade>> = pick_random_upgrades();

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

    let max_dash_cooldown = Duration::from_secs(3).as_millis();
    let mut dash_cooldown = Duration::from_secs(0).as_millis();
    let mut is_dashing = false;

    // Level Transition tweener
    let (start, end) = (0., 1.);
    let duration = 2.0;
    let mut tweener = Tweener::sine_in_out(start, end, duration);
    const DT: f32 = 1.0 / 60.0;

    // Level up title tweener
    let (lup_t_start, lup_t_end) = (0. as f32, 10. as f32);
    let duration = 2.0;
    let lup_tweener = Tweener::sine_in_out(lup_t_start, lup_t_end, duration);
    let mut looper = Oscillator::new(lup_tweener);

    let mut damage_popups : Vec<DamagePopup> = Vec::new();
    let mut sw = stopwatch_rs::StopWatch::start();

    loop {
        clear_background(Color::from_rgba(37, 33, 41, 255));

        let camera_buffer = (screen_height() / camera_zoom) * 2.0 * 0.1;
        camera_focal_y = player_pos_y;
        camera_focal_x = player_pos_x;

        set_camera(&Camera2D {
            target: vec2(camera_focal_x + 4., camera_focal_y + 4.),
            zoom: Vec2::new(camera_zoom / screen_width() * 2., -camera_zoom / screen_height() * 2.),
            // rotation: (camera_target_angle - camera_angle) * t + camera_angle,
            ..Default::default()
        });

        match level_state {
            LevelState::InGame => {
                choosen_upgrade_index = 0;
                // println!("mins {}", (get_minutes_from_millis(sw.split().split.as_millis())));
                // println!("secs {}", (get_seconds_from_millis(sw.split().split.as_millis())));

                for x in 0..80 {
                    for y in 0..50 {
                        draw_map_cell(main_texture, x, y);
                    }
                }
        
                // Revisit dashing
                // if is_key_pressed(KeyCode::Z) {
                //     is_dashing = true;
                //     dash_cooldown = max_dash_cooldown;
                // }
                // if dash_cooldown <= 0 {
                //     // spawn_enemies(&mut enemies, &player_pos_x, &player_pos_y);
                //     is_dashing = false;
                // } else {
                //     dash_cooldown = dash_cooldown.clamp(0, dash_cooldown - 100);
                // }

                move_player(&mut player_pos_x, &mut player_pos_y, &mut player_flip_x, &player_speed_bonus, &is_dashing);
                // dash_player(&mut player_pos_x, &mut player_pos_y, &is_dashing);

                update_enemies_position(&mut enemies, &mut player_pos_x, &mut player_pos_y);
                update_enemies_pushing(&mut enemies);
                update_enemies_colliding(&mut enemies, &mut player_pos_x, &mut player_pos_y, &mut player_hp, &mut player_inv_timer);
                update_dead_enemies(&mut dead_enemies, &mut player_pos_x);
                update_bullets(&mut bullets, &mut enemies);

                draw_player(main_texture, &mut player_pos_x, &mut player_pos_y, &player_flip_x, &player_inv_timer);
                draw_enemies(main_texture, &mut enemies, &mut player_pos_x, &mut player_pos_y);
                draw_dead_enemies(main_texture, &mut dead_enemies, &mut player_pos_x, &mut player_pos_y);
                draw_bullets(main_texture, &mut bullets);

                // draw_player_collider(&mut player_pos_x, &mut player_pos_y);
                // draw_enemies_collider(&mut enemies);

                for popup in damage_popups.iter_mut() {
                    popup.update();
                    popup.draw(ui_texture);
                }

                // Spawning code
                if enemy_cooldown <= 0 {
                    spawn_enemies(&mut enemies, &player_pos_x, &player_pos_y);
                    enemy_cooldown = max_enemy_cooldown;
                } else {
                    enemy_cooldown = enemy_cooldown.clamp(0, enemy_cooldown - 100);
                }

                damage_enemy(&mut bullets, &mut enemies, &mut player_xp, &mut damage_popups);
                kill_enemies(&mut enemies, &mut player_xp, &mut dead_enemies);

                if player_xp >= player_max_xp {
                    upgrades = pick_random_upgrades();
                    level_up_player(&mut player_xp, &mut player_max_xp, &mut player_level, &mut level_state);
                }
                
                if bullet_cooldown <= 0 {
                    spawn_bullet(&mut bullets, &mut enemies, &mut player_pos_x, &mut player_pos_y);
                    bullet_cooldown = max_cooldown;
                } else {
                    bullet_cooldown = bullet_cooldown.clamp(0, bullet_cooldown - 100);
                }
        
                // Get rid of things that shouldn't be around anymore
                // Bullets, enemies, particles, pop-ups
                bullets.retain(|b| b.active);
                enemies.retain(|e| e.alive);
                dead_enemies.retain(|e| e.alive);
                damage_popups.retain(|e| e.active);          

                set_default_camera();

                current_player_hp_percentage = (player_hp / player_max_hp) * 100.;
                current_player_xp_percentage = (player_xp / player_max_xp) * 100.;
                draw_level_ui(ui_texture, &current_player_hp_percentage, &current_player_xp_percentage, &player_level, &player_inv_timer);
                draw_level_timer_ui(
                    font, 
                    get_minutes_from_millis(sw.split().split.as_millis()), 
                    get_seconds_from_millis(sw.split().split.as_millis())
                );

                // Trigger level progression
                // if sw.split().split.as_millis() > 5000 {
                //     draw_rectangle(
                //         0., 0., screen_width(), screen_height(), 
                //         Color::new(0., 0., 0., tweener.move_by(DT)), 
                //     );
                //     if tweener.is_finished() {
                //         level_state = LevelState::StageCleared
                //     }
                // }
            },
            LevelState::StageCleared => {
                clear_background(BLACK);
                // clear vecs/entities
                // reset defaults
                // next stage?
            }
            LevelState::LevelUp => {
                for x in 0..80 {
                    for y in 0..50 {
                        draw_map_cell(main_texture, x, y);
                    }
                }
        
                sw.suspend();
                draw_player(main_texture, &mut player_pos_x, &mut player_pos_y, &player_flip_x, &player_inv_timer);
                // draw_player_collider(&mut player_pos_x, &mut player_pos_y);
                draw_enemies(main_texture, &mut enemies, &mut player_pos_x, &mut player_pos_y);
                draw_enemies_collider(&mut enemies);
                draw_bullets(main_texture, &mut bullets);

                // In-level UI
                draw_rectangle(0., screen_height() - 80., screen_width(), 120., BLACK);
                set_default_camera();

                choose_upgrade_input(&mut choosen_upgrade_index, &mut upgrade_menu_tween);
                let result = level_up_input(&mut level_state);
                if let Some(newstate) = result {
                    // fine tune!
                    let idx = choosen_upgrade_index as usize;
                    let upg = upgrades[idx].get_name();
                    match upg {
                        "Speed" => {
                            println!("Speed upgrade");
                            player_speed_bonus += 0.1;
                        }
                        "FireRate" => {
                            println!("FireRate upgrade");
                        }
                        "Recovery" => {
                            println!("Recovery upgrade");
                        }
                        _ => {}
                    }
                    sw.resume();
                    level_state = newstate;
                }

                current_player_hp_percentage = (player_hp / player_max_hp) * 100.;
                current_player_xp_percentage = (player_xp / player_max_xp) * 100.;
                draw_level_ui(ui_texture, &current_player_hp_percentage, &current_player_xp_percentage, &player_level, &player_inv_timer);
                draw_level_up(&choosen_upgrade_index, &upgrades, upgrade_texture, &mut upgrade_menu_tween);
                draw_level_up_title(font, &mut looper);          
            }
        }

        next_frame().await;
    }
}
