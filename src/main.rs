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

// ============================================================================
// GAME STATE STRUCTS
// ============================================================================

pub struct Player {
    pub pos_x: f32,
    pub pos_y: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub xp: f32,
    pub max_xp: f32,
    pub level: i32,
    pub flip_x: bool,
    pub speed_bonus: f32,
    pub regen: f32,
    pub regen_timer: Timer,
    pub inv_timer: Timer,
    pub is_dashing: bool,
    pub direction: Option<Direction>,
    pub dashing_timer: Timer,
    pub dash_speed: f32,
    pub active: bool,
    pub damage: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos_x: 128.,
            pos_y: 128.,
            hp: 100.,
            max_hp: 100.,
            xp: 0.,
            max_xp: 100.,
            level: 1,
            flip_x: false,
            speed_bonus: 1.,
            regen: 1.,
            regen_timer: Timer::new(5000),
            inv_timer: Timer::new(1800),
            is_dashing: false,
            direction: None,
            dashing_timer: Timer::new(500),
            dash_speed: 40.0,
            active: true,
            damage: 2.,
        }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.hp = (self.hp - amount).max(0.);
    }

    pub fn heal(&mut self, amount: f32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    pub fn add_xp(&mut self, amount: f32) {
        self.xp += amount;
    }

    pub fn level_up(&mut self) {
        self.xp = 0.;
        self.level += 1;
    }

    pub fn position(&self) -> Position {
        Position {
            x: self.pos_x,
            y: self.pos_y,
        }
    }
}

pub struct World {
    pub enemies: Vec<Enemies>,
    pub bat_enemies: Vec<BatEnemy>,
    pub tower_enemies: Vec<TowerEnemy>,
    pub bullets: Vec<Bullet>,
    pub enemy_bullets: Vec<Bullet>,
    pub dead_enemies: Vec<DeadEnemy>,
    pub damage_popups: Vec<DamagePopup>,
    pub particles: Vec<Particle>,
    pub intro_particles: Vec<Particle>,
    pub progression: f32,
    pub base_given_xp: f32,
    pub kill_count: i32,
    pub max_b_cooldown: Timer,
    pub bullet_cooldown: Timer,
    pub current_bullet_cooldown_bonus: f32,
    pub screen_shake_amount: f32,
    pub stopwatch: Stopwatch,
}

impl World {
    pub fn new() -> Self {
        World {
            enemies: Vec::new(),
            bat_enemies: Vec::new(),
            tower_enemies: Vec::new(),
            bullets: Vec::new(),
            enemy_bullets: Vec::new(),
            dead_enemies: Vec::new(),
            damage_popups: Vec::new(),
            particles: Vec::new(),
            intro_particles: Vec::new(),
            progression: 1.0,
            base_given_xp: 50.0,
            kill_count: 0,
            max_b_cooldown: Timer::new(700),
            bullet_cooldown: Timer::new(700),
            current_bullet_cooldown_bonus: 1.0,
            screen_shake_amount: 0.,
            stopwatch: Stopwatch::new(),
        }
    }

    pub fn reset(&mut self) {
        self.enemies.clear();
        self.bat_enemies.clear();
        self.tower_enemies.clear();
        self.bullets.clear();
        self.enemy_bullets.clear();
        self.dead_enemies.clear();
        self.damage_popups.clear();
        self.particles.clear();
        self.progression = 1.0;
        self.base_given_xp = 50.0;
        self.kill_count = 0;
        self.bullet_cooldown = Timer::new(700);
        self.current_bullet_cooldown_bonus = 1.0;
        self.screen_shake_amount = 0.;
        self.stopwatch = Stopwatch::new();
    }

    pub fn apply_screen_shake(&mut self, amount: f32) {
        self.screen_shake_amount += amount;
    }

    pub fn decay_screen_shake(&mut self) {
        self.screen_shake_amount *= 0.94;
    }
}

pub struct Renderer {
    pub anims: HashMap<String, Animation>,
    pub tweener: Tweener<f32, f32, Box<dyn ::tween::Tween<f32>>>,
    pub test_tweener: TestTween<f32, f32>,
    pub level_up_letters_tweener: TestTween<f32, f32>,
    pub main_title_tweener: TestTween<f32, f32>,
    pub init_upgrade_tweener: TestTween<f32, f32>,
    pub death_tweener: Tweener<f32, f32, Box<dyn ::tween::Tween<f32>>>,
    pub upgrade_menu_tween: Tween,
    pub choosen_upgrade_index: i32,
}

impl Renderer {
    pub fn new() -> Self {
        let mut idle_state_rects: Vec<Rect> = Vec::new();
        idle_state_rects.push(Rect::new(1., 1., 9., 9.));
        idle_state_rects.push(Rect::new(10., 1., 9., 9.));
        idle_state_rects.push(Rect::new(19., 1., 9., 9.));
        idle_state_rects.push(Rect::new(28., 1., 9., 9.));

        let idle_state_frame_lenghts: Vec<Duration> =
            vec![Duration::from_millis(200); idle_state_rects.len()];

        let idle_animation = Animation {
            frames: idle_state_rects.clone(),
            frame_length: idle_state_frame_lenghts.clone(),
            anim_duration: Duration::from_secs(0),
            current_frame: 0,
            current_frame_length: idle_state_frame_lenghts[0],
            repeating: true,
        };

        let mut anims = HashMap::new();
        anims.insert("idle".to_string(), idle_animation);

        Renderer {
            anims,
            tweener: Tweener::new(0., screen_width(), 5.0, Box::new(::tween::SineInOut)),
            test_tweener: Tweener::new(0., 10., 1.5, Box::new(Oscillator::new(CircInOut))),
            level_up_letters_tweener: Tweener::new(
                -3.,
                3.,
                2.2,
                Box::new(Oscillator::new(::tween::SineInOut)),
            ),
            main_title_tweener: Tweener::new(
                -10.,
                10.,
                2.2,
                Box::new(Oscillator::new(::tween::SineInOut)),
            ),
            init_upgrade_tweener: Tweener::new(0., 10., 1.5, Box::new(CircInOut)),
            death_tweener: Tweener::new(0., 10., 2., Box::new(::tween::SineInOut)),
            upgrade_menu_tween: Tween::from_keyframes(
                vec![
                    Keyframe::new(0.0, 0.0, EaseOut),
                    Keyframe::new(4.0, 0.5, EaseOut),
                    Keyframe::new(0.0, 1.0, EaseOut),
                ],
                0,
                1,
                true,
            ),
            choosen_upgrade_index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.tweener = Tweener::new(0., screen_width(), 5.0, Box::new(::tween::SineInOut));
        self.test_tweener = Tweener::new(0., 10., 1.5, Box::new(Oscillator::new(CircInOut)));
        self.level_up_letters_tweener = Tweener::new(
            -3.,
            3.,
            2.2,
            Box::new(Oscillator::new(::tween::SineInOut)),
        );
        self.init_upgrade_tweener = Tweener::new(0., 10., 1.5, Box::new(CircInOut));
        self.death_tweener = Tweener::new(0., 10., 2., Box::new(::tween::SineInOut));
        self.upgrade_menu_tween = Tween::from_keyframes(
            vec![
                Keyframe::new(0.0, 0.0, EaseOut),
                Keyframe::new(4.0, 0.5, EaseOut),
                Keyframe::new(0.0, 1.0, EaseOut),
            ],
            0,
            1,
            true,
        );
        self.choosen_upgrade_index = 0;
    }
}

pub struct GameSession {
    pub player: Player,
    pub world: World,
    pub renderer: Renderer,
    pub main_texture: Texture2D,
    pub ui_texture: Texture2D,
    pub upgrade_texture: Texture2D,
    pub player_texture: Texture2D,
    pub slime_texture: Texture2D,
    pub main_title_texture: Texture2D,
    pub font: Font,
}

impl GameSession {
    pub async fn new() -> Result<Self, macroquad::prelude::FileError> {
        Ok(GameSession {
            player: Player::new(),
            world: World::new(),
            renderer: Renderer::new(),
            main_texture: load_texture("assets/vs-dx-atlas-padded.png").await.unwrap(),
            ui_texture: load_texture("assets/vs-dx-ui-atlas.png").await.unwrap(),
            upgrade_texture: load_texture("assets/vs-dx-upgrades-atlas.png").await.unwrap(),
            player_texture: load_texture("assets/vs-dx-player-atlas.png").await.unwrap(),
            slime_texture: load_texture("assets/vs-dx-enemies-atlas.png").await.unwrap(),
            main_title_texture: load_texture("assets/vs-dx-maintitle-atlas.png").await.unwrap(),
            font: load_ttf_font("assets/smolFontMono.ttf").await.unwrap(),
        })
    }

    pub fn reset(&mut self) {
        self.player = Player::new();
        self.world.reset();
        self.renderer.reset();
    }

    pub fn setup_textures(&self) {
        self.main_texture.set_filter(FilterMode::Nearest);
        self.ui_texture.set_filter(FilterMode::Nearest);
        self.upgrade_texture.set_filter(FilterMode::Nearest);
        self.font.set_filter(FilterMode::Nearest);
        self.player_texture.set_filter(FilterMode::Nearest);
        self.slime_texture.set_filter(FilterMode::Nearest);
        self.main_title_texture.set_filter(FilterMode::Nearest);
    }
}

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

fn draw_player(texture: Texture2D, frame: Option<Rect>, player: &Player) {
    let mut color = WHITE;
    if player.inv_timer.value() != 1.0 {
        color = Color::new(1.0, 0., 0., 1.);
    }
    draw_texture_ex(
        texture,
        player.pos_x,
        player.pos_y,
        color,
        DrawTextureParams {
            dest_size: Some(vec2(8., 8.)),
            source: frame,
            flip_x: player.flip_x,
            ..Default::default()
        },
    )
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
    return (x*x+y*y)<r*r
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

fn move_player(player: &mut Player, delta: f32) {
    let a = axis(is_key_down(KeyCode::Left), is_key_down(KeyCode::Right));
    let b = axis(is_key_down(KeyCode::Down), is_key_down(KeyCode::Up));
    let magnitude = (a.powi(2) + b.powi(2)).sqrt();
    let foo_x = a / magnitude;
    let foo_y = b / magnitude;
    if a != 0. || b != 0. {
        player.pos_x += foo_x * delta * PLAYER_SPEED * player.speed_bonus;
        player.pos_y -= foo_y * delta * PLAYER_SPEED * player.speed_bonus;
    }

    if a == -1. {
        player.flip_x = true
    }
    if a == 1. {
        player.flip_x = false
    }
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
    let camera_zoom: f32 = 10.0;

    let mut session = GameSession::new().await.unwrap();
    session.setup_textures();

    let mut upgrades: Vec<Box<dyn Upgrade>> = pick_random_upgrades();
    let mut level_state = LevelState::PreGame;

    loop {
        clear_background(Color::from_rgba(37, 33, 41, 255));
        let delta = get_frame_time();

        session.world.decay_screen_shake();

        let screen_shake = Vec2::new(
            rand::gen_range(-session.world.screen_shake_amount, session.world.screen_shake_amount),
            rand::gen_range(-session.world.screen_shake_amount, session.world.screen_shake_amount),
        );
        let camera_focal_y = session.player.pos_y;
        let camera_focal_x = session.player.pos_x;
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

                draw_particles(&mut session.world.intro_particles);
                draw_texture_ex(session.main_title_texture, screen_width()/2.-180., 100. + session.renderer.main_title_tweener.move_by(delta), WHITE, 
                    DrawTextureParams { 
                        dest_size: Some(vec2(36. * 10., 22. * 10.)), 
                        source: Some(Rect::new(5., 3., 36., 22.)),
                        ..Default::default()
                    }
                );
                draw_texture_ex(session.main_title_texture, screen_width()/2.-290., screen_height() - 300., WHITE, 
                    DrawTextureParams { 
                        dest_size: Some(vec2(58. * 10., 18. * 10.)), 
                        source: Some(Rect::new(5., 30., 58., 18.)),
                        ..Default::default()
                    }
                );
                spawn_particle(
                    &mut session.world.intro_particles, 
                    screen_width()/4., 
                    0.,
                    Box::new(IntroParticle{})
                );
                update_particles(&mut session.world.intro_particles);
                                
                if is_key_pressed(KeyCode::Z) {
                    // restart the "game state"
                    session.reset();
                    level_state = LevelState::InGame;
                }
                // tween to start
            }
            LevelState::InGame => {
                session.world.stopwatch.tick(Duration::from_secs_f32(0.01));
                session.renderer.choosen_upgrade_index = 0;
                for x in 0..160 {
                    for y in 0..100 {
                        draw_map_cell(session.main_texture, x, y);
                    }
                }
                
                // Update block
                if session.player.active {
                    // Move and Dashing input block
                    if is_key_pressed(KeyCode::X) && !session.player.is_dashing {
                        session.player.direction = get_direction();
                        if let Some(_dir) = session.player.direction {
                            session.player.dashing_timer.restart();
                            session.player.is_dashing = true;
                        }
                    }
                    if !session.player.is_dashing {
                        move_player(&mut session.player, delta);
                    }
                    update_enemies_position(&mut session.world.enemies, &mut session.player.pos_x, &mut session.player.pos_y);
                    update_enemies_pushing(&mut session.world.enemies);
                    update_enemies_colliding(&mut session.world.enemies, &mut session.player.pos_x, &mut session.player.pos_y, &mut session.player.hp, &session.player.is_dashing, &mut session.player.inv_timer, &mut session.world.screen_shake_amount);
                    update_bat_enemies_position(&mut session.world.bat_enemies);
                    update_bat_enemies_colliding(
                        &mut session.world.bat_enemies, 
                        &mut session.player.pos_x, 
                        &mut session.player.pos_y, 
                        &mut session.player.hp, 
                        &mut session.player.inv_timer, 
                        &mut session.world.screen_shake_amount, 
                        &mut session.world.damage_popups,
                        &session.player.is_dashing
                    );
                    update_tower_enemies(&mut session.world.tower_enemies, &session.player.pos_x, &session.player.pos_y, &mut session.world.enemy_bullets);
                    update_bullets(&mut session.world.bullets, &mut session.world.particles);
                    update_enemy_bullets(&mut session.world.enemy_bullets, &mut session.world.particles, delta);
                }
                update_dead_enemies(&mut session.world.dead_enemies, &mut session.player.pos_x);
                update_particles(&mut session.world.particles);
                
                if session.player.is_dashing {        
                    // Calculate the dash distance based on the dash speed and delta time
                    let dash_distance = session.player.dash_speed * delta;
        
                    // Update player position based on dash direction
                    // 0.7071 was pure experimentation
                    match session.player.direction {
                        Some(Direction::Up) => session.player.pos_y -= dash_distance,
                        Some(Direction::Down) => session.player.pos_y += dash_distance,
                        Some(Direction::Left) => session.player.pos_x -= dash_distance,
                        Some(Direction::Right) => session.player.pos_x += dash_distance,
                        Some(Direction::UpLeft) => {
                            session.player.pos_x -= dash_distance * 0.7071;
                            session.player.pos_y -= dash_distance * 0.7071;
                        }
                        Some(Direction::UpRight) => {
                            session.player.pos_x += dash_distance * 0.7071;
                            session.player.pos_y -= dash_distance * 0.7071;
                        }
                        Some(Direction::DownLeft) => {
                            session.player.pos_x -= dash_distance * 0.7071;
                            session.player.pos_y += dash_distance * 0.7071;
                        }
                        Some(Direction::DownRight) => {
                            session.player.pos_x += dash_distance * 0.7071;
                            session.player.pos_y += dash_distance * 0.7071;
                        }
                        None => {}
                    }

                    spawn_particle(
                        &mut session.world.particles, 
                        session.player.pos_x, 
                        session.player.pos_y,
                        Box::new(PlayerDashParticle{ texture: session.player_texture})
                    );

                    if session.player.dashing_timer.finished() {
                        session.player.is_dashing = false;
                    }
                }

                let player_frame = session.renderer.anims.get_mut("idle").unwrap().get_animation_source(Duration::from_secs_f32(get_frame_time()));
                
                // Draw block
                draw_particles(&mut session.world.particles);
                // player.draw(player_texture, frame);
                draw_player(
                    session.player_texture,
                    player_frame, 
                    &session.player
                );
                draw_enemies(
                    session.slime_texture, 
                    &mut session.world.enemies, 
                    &mut session.player.pos_x, 
                    &mut session.player.pos_y
                );
                draw_tower_enemies(session.slime_texture, &mut session.world.tower_enemies);
                draw_bat_enemies(
                    session.slime_texture,
                    &mut session.world.bat_enemies
                );
                draw_dead_enemies(session.slime_texture, &mut session.world.dead_enemies, &mut session.player.pos_x, &mut session.player.pos_y);
                draw_bullets(session.main_texture, &mut session.world.bullets);
                draw_enemy_bullets(session.main_texture, &mut session.world.enemy_bullets);

                // draw_player_collider(&mut session.player.pos_x, &mut session.player.pos_y);
                // draw_enemies_collider(&mut session.world.enemies);

                for popup in session.world.damage_popups.iter_mut() {
                    popup.update();
                    popup.draw(session.font);
                }

                // Spawning enemies
                // Count slimes
                if session.player.active {
                    if session.world.enemies.len() < (5*(session.world.progression as usize)) {
                        let mut given_xp = session.world.base_given_xp - (0.1 * (session.world.base_given_xp)) - (0.5 * (session.world.kill_count as f32)) - session.world.progression*4.;
                        if given_xp < 3. { given_xp = 3. }
                        // println!("{} given_xp", given_xp);                    
                        spawn_enemies(&mut session.world.enemies, &session.player.pos_x, &session.player.pos_y, given_xp);
                    }

                    // Count Bats
                    if session.world.bat_enemies.len() < (2*(session.world.progression as usize)) {
                        let x_dir: f32;
                        match rand::gen_range(0, 2) {
                            0 => x_dir = -1.,
                            _ => x_dir = 1.,
                        }
                        let spawn_pos_y = session.player.pos_y * (rand::gen_range(0.5, 2.));
                        let mut given_xp = session.world.base_given_xp - (0.1 * (session.world.base_given_xp)) - (0.5 * (session.world.kill_count as f32)) - session.world.progression*4.;
                        if given_xp < 3. { given_xp = 3. }
                        println!("{} given_xp", given_xp);
                        session.world.bat_enemies.push(
                            BatEnemy::new(session.player.pos_x - 64. * (x_dir), spawn_pos_y, x_dir, given_xp)
                        );
                    }
                    // And towers
                    if session.world.progression >= 3. {
                        if session.world.tower_enemies.len() < (2*session.world.progression as usize) {
                            // Random around radius
                            // let angle = rand::gen_range(0.0, std::f32::consts::TAU); // Random angle in radians
                            // let distance = rand::gen_range(20.,100.); // Random distance within the spawn radius
                        
                            // let spawn_x = session.player.pos_x + distance * angle.cos();
                            // let spawn_y = session.player.pos_y + distance * angle.sin();
                        
                            // Random around circ 
                            let angle = rand::gen_range(0.0,std::f32::consts::TAU); // Random angle in radians
        
                            let spawn_x = session.player.pos_x + 32. * angle.cos();
                            let spawn_y = session.player.pos_y + 32. * angle.sin();
        
                            session.world.tower_enemies.push(
                                TowerEnemy::new(spawn_x, spawn_y)
                            );
                        }
                    }
                }

                if session.player.regen_timer.finished() && session.player.active {
                    if session.player.hp + session.player.regen >= session.player.max_hp {
                        session.player.hp = session.player.max_hp
                    } else {
                        session.player.hp += session.player.regen;
                    }
                    session.player.regen_timer.restart();
                }
 
                damage_enemy(&mut session.world.bullets, &mut session.world.enemies, &mut session.world.damage_popups, &mut session.world.screen_shake_amount, &session.player.damage);
                bullet_damage_player(&mut session.world.enemy_bullets, &session.player.pos_x, &session.player.pos_y, &mut session.player.hp, &mut session.world.damage_popups, &mut session.world.screen_shake_amount, &mut session.player.inv_timer, &session.player.is_dashing);
                kill_enemies(&mut session.world.enemies, &mut session.player.xp, &mut session.world.dead_enemies, &mut session.world.kill_count, &mut session.world.progression, &mut session.world.base_given_xp);
                kill_bat_enemies(&mut session.world.bat_enemies, &mut session.player.xp, &mut session.world.dead_enemies, &mut session.world.kill_count, &mut session.world.progression, &mut session.world.base_given_xp);
                clean_bat_enemies(&mut session.world.bat_enemies, &session.player.pos_x, &session.player.pos_y);

                if session.player.xp >= session.player.max_xp {
                    upgrades = pick_random_upgrades();
                    level_up_player(&mut session.player.xp, &mut session.player.max_xp, &mut session.player.level, &mut level_state);
                }
                
                if session.world.bullet_cooldown.finished() {
                    spawn_bullet(&mut session.world.bullets, &mut session.world.enemies, &mut session.player.pos_x, &mut session.player.pos_y);
                    session.world.bullet_cooldown.set_duration_millis(((3000 as f32) * session.world.current_bullet_cooldown_bonus) as u64);
                    session.world.bullet_cooldown.restart();
                }

                // Get rid of things that shouldn't be around anymore
                // Bullets, enemies, particles, pop-ups
                session.world.bullets.retain(|b| b.active);
                session.world.enemy_bullets.retain(|b| b.active);
                session.world.enemies.retain(|e| e.alive);
                session.world.bat_enemies.retain(|e| e.active);
                session.world.tower_enemies.retain(|e| e.active);
                session.world.dead_enemies.retain(|e| e.active);
                session.world.damage_popups.retain(|e| e.active);
                session.world.particles.retain(|p| p.active);        

                set_default_camera();

                let current_player_hp_percentage = (session.player.hp / session.player.max_hp) * 100.;
                let current_player_xp_percentage = (session.player.xp / session.player.max_xp) * 100.;
                draw_level_ui(session.ui_texture, &current_player_hp_percentage, &current_player_xp_percentage, &session.player.level, &session.player.inv_timer);
                draw_level_timer_ui(
                    session.font, 
                    get_minutes_from_millis(session.world.stopwatch.elapsed().as_millis()), 
                    get_seconds_from_millis(session.world.stopwatch.elapsed().as_millis())
                );

                if session.player.hp <= 0. {
                    session.renderer.death_tweener.move_by(delta);
                    session.player.hp = 0.;
                    session.player.active = false;
                    session.world.stopwatch.pause();

                    session.world.enemies = Vec::new();
                    session.world.bat_enemies = Vec::new();
                    session.world.bullets = Vec::new();
                    session.world.dead_enemies = Vec::new();
                    session.world.damage_popups = Vec::new();
                    session.world.particles = Vec::new();

                    session.player.pos_x = -999.;
                    session.player.pos_y = -999.;


                    session.world.screen_shake_amount += 0.5 * 1.1;

                    if session.renderer.death_tweener.is_finished() {
                        level_state = LevelState::PreGame;
                    }
                }

                // Trigger end game progression
                if session.world.stopwatch.elapsed().as_millis() > 240000 && session.player.active {
                    // destroy all entities 
                    // but the player
                    // - deallocates but not sure if its good
                    session.world.enemies = Vec::new();
                    session.world.bat_enemies = Vec::new();
                    session.world.bullets = Vec::new();
                    session.world.dead_enemies = Vec::new();
                    session.world.damage_popups = Vec::new();
                    session.world.particles = Vec::new();

                    if session.world.stopwatch.elapsed().as_millis() < 246000 {
                        session.world.screen_shake_amount += 0.5;
                    }

                    if session.world.stopwatch.elapsed().as_millis() > 248000 {
                        draw_rectangle(
                            0., 
                            0., 
                            session.renderer.tweener.move_by(delta), 
                            screen_height(), 
                            Color::from_rgba(37, 33, 41, 255)
                        );
                        if session.renderer.tweener.is_finished() {
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
                    TextParams { font: session.font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
                );
            
                draw_text_ex(
                    "Thanks for playing!",
                    (screen_width() / 2.) - 300., 
                    250., 
                    TextParams { font: session.font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
                );

                draw_text_ex(
                    "Made by inacho",
                    (screen_width() / 2.) - 230., 
                    350., 
                    TextParams { font: session.font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
                );

                draw_text_ex(
                    "For LowRezJam2023",
                    (screen_width() / 2.) - 280., 
                    400., 
                    TextParams { font: session.font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
                );                

                draw_text_ex(
                    "Press Z to restart",
                    (screen_width() / 2.) - 300., 
                    500., 
                    TextParams { font: session.font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
                );  

                if is_key_pressed(KeyCode::Z) {
                    level_state = LevelState::PreGame;
                }
            }
            LevelState::LevelUp => {
                for x in 0..80 {
                    for y in 0..50 {
                        draw_map_cell(session.main_texture, x, y);
                    }
                }
        
                session.world.stopwatch.pause();
                let frame = session.renderer.anims.get_mut("idle").unwrap().get_animation_source(Duration::from_secs_f32(get_frame_time()));
                draw_player(session.player_texture, frame, &session.player);
                // draw_player_collider(&mut session.player.pos_x, &mut session.player.pos_y);
                draw_enemies(session.slime_texture, &mut session.world.enemies, &mut session.player.pos_x, &mut session.player.pos_y);
                draw_enemies_collider(&mut session.world.enemies);
                draw_bullets(session.main_texture, &mut session.world.bullets);

                // In-level UI
                draw_rectangle(0., screen_height() - 80., screen_width(), 120., BLACK);
                set_default_camera();

                choose_upgrade_input(&mut session.renderer.choosen_upgrade_index, &mut session.renderer.upgrade_menu_tween);
                let result = level_up_input();
                if let Some(newstate) = result {
                    // fine tune!
                    let idx = session.renderer.choosen_upgrade_index as usize;
                    let upg = upgrades[idx].get_name();
                    match upg {
                        "Speed" => {
                            // println!("Speed upgrade");
                            session.player.speed_bonus += 0.1;
                        }
                        "FireRate" => {
                            session.world.current_bullet_cooldown_bonus -= 0.1;
                            // println!("FireRate upgrade");
                        }
                        "Recovery" => {
                            session.player.regen += 2.;
                            // println!("Recovery upgrade");
                        }
                        "FasterRecovery" => {
                            let dur = session.player.regen_timer.duration;
                            let dur_5_percent = (session.player.regen_timer.duration.as_millis() as f32) * 0.05;
                            // println!("recovery {} new recovery {}, 5percent {}", dur.as_millis(), (dur.as_millis() - (dur_5_percent.round() as u128)), dur_5_percent);
                            session.player.regen_timer.set_duration_millis((dur.as_millis() - (dur_5_percent.round() as u128)) as u64);
                        }
                        "Dash" => {
                            let dash_5_percent = session.player.dash_speed * 0.05;
                            session.player.dash_speed += dash_5_percent;
                            // println!("{}", dash_speed);
                        },
                        "MoreIframes" => {
                            let dur = session.player.inv_timer.duration;
                            let extra = (session.player.inv_timer.duration.as_millis() as f32) * 0.1;
                            // println!("recovery {} new recovery {}, 5percent {}", dur.as_millis(), (dur.as_millis() + (extra.round() as u128)), extra);
                            session.player.inv_timer.set_duration_millis((dur.as_millis() + (extra.round() as u128)) as u64);
                        }
                        _ => {}
                    }
                    session.world.stopwatch.unpause();
                    level_state = newstate;
                }

                let current_player_hp_percentage = (session.player.hp / session.player.max_hp) * 100.;
                let current_player_xp_percentage = (session.player.xp / session.player.max_xp) * 100.;
                draw_level_ui(session.ui_texture, &current_player_hp_percentage, &current_player_xp_percentage, &session.player.level, &session.player.inv_timer);
                draw_level_up(
                    &session.renderer.choosen_upgrade_index, 
                    &upgrades, 
                    session.font, 
                    &mut session.renderer.upgrade_menu_tween, 
                    &mut session.renderer.init_upgrade_tweener
                );
                draw_level_up_title(session.font, &mut session.renderer.test_tweener, &mut session.renderer.level_up_letters_tweener);
            }
        }

        next_frame().await;
    }
}