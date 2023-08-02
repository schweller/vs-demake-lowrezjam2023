use std::time::Duration;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf { 
        window_title: "Rustlike".to_owned(), 
        window_width: 640, // 640 + 120 
        window_height: 640, // 320 + 120
        ..Default::default()
    }
}

#[derive(Clone, Copy)]
pub struct Enemies {
    pub position: Position,
    pub collider: Collider
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

fn draw_all(texture: Texture2D) {}

fn sprite_rect(ix: u32) -> Rect {
    let sw = 8. as f32;
    let sh = 8. as f32;
    let sx = (ix % 5) as f32 * (sw + 2 as f32) + 2 as f32;
    let sy = (ix / 5) as f32 * (sh + 2 as f32) + 2 as f32;

    // TODO: configure tiles margin
    Rect::new(sx + 1., sy + 1., sw - 2.2, sh - 2.2)
}

fn draw_player(texture: Texture2D, x: &mut f32, y: &mut f32, flip_x: &bool) {
    draw_texture_ex(
        texture, 
        *x,
        *y, 
        WHITE,
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

fn update_enemies_position(enemies: &mut Vec<Enemies>, x: &mut f32, y: &mut f32) {
    let delta = get_frame_time();
    let player_vec: Vec2 = Vec2::new(*x, *y);

    for e in enemies.iter_mut() {
        // x.atan2(other);
        let enem_vec = Vec2::new(e.position.x, e.position.y);
        let dir = enem_vec - player_vec;
        e.position.x -= dir.x * delta * 0.1; 
        e.position.y -= dir.y * delta * 0.1;
    }
}

fn col(a: Position, b: Position, r: f32) -> bool {
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

fn dist(a: Position, b: Position, r: f32) -> f32 {
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

fn get_dir_(vec1: Position, vec2: Position) -> f32 {
    return (vec2.x - vec1.x).atan2(vec2.y - vec1.y);
}

fn update_enemies_pushing(enemies: &mut Vec<Enemies>) {
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

fn draw_enemies(texture: Texture2D, enemies: &mut Vec<Enemies>, x: &mut f32, y: &mut f32) {
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

fn draw_enemies_collider(enemies: &mut Vec<Enemies>) {
    for e in enemies.iter() {
        draw_circle(
            e.position.x + 4., 
            e.position.y + 4., 
            e.collider.radius, 
            Color::from_rgba(255, 0, 0, 60)
        );
    }
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
    let mut _dir = Vec2::new(-1., -1.);
    for e in enemies.iter() {
        let _d = dist(
            Position { x: *x, y: *y },
            e.position,
        _dist);
        if _d < _dist {
            _dist= _d;
            // _dir = get_dir(e.position.x,e.position.y,*x,*y);
            _dir = Vec2::new(*x, *y) - Vec2::new(e.position.x, e.position.y);
        }
    }
    bullets.push(Bullet { x: *x + 2., y: *y + 2., dir_x: _dir.x, dir_y: _dir.y, active: true });
}

fn update_bullets(bullets: &mut Vec<Bullet>, enemies: &mut Vec<Enemies>) {
    let delta = get_frame_time();
    for bullet in bullets.iter_mut() {
        if bullet.active {
            bullet.x -= bullet.dir_x * delta * 0.5; 
            bullet.y -= bullet.dir_y * delta * 0.5;
        }
    }

    for e in enemies.iter() {
        for bullet in bullets.iter_mut() {
            if col(
                Position { x: bullet.x, y: bullet.y }, 
                Position { x: e.position.x + 2., y: e.position.y + 2. }, 
                5.
            ) {
                bullet.active = false;
            }
        }
    }
}

const PLAYER_SPEED: f32 = 20.;

fn move_player(x: &mut f32, y: &mut f32, flip_x: &mut bool) {
    let delta = get_frame_time();
    if is_key_down(KeyCode::Left) {
        *x -= PLAYER_SPEED * delta;
        *flip_x = true;
    }
    if is_key_down(KeyCode::Right) {
        *x += PLAYER_SPEED * delta;
        *flip_x = false;
    }
    if is_key_down(KeyCode::Up) {
        *y -= PLAYER_SPEED * delta;
    }
    if is_key_down(KeyCode::Down) {
        *y += PLAYER_SPEED * delta;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let min_camera_zoom = 1.3;
    let max_camera_zoom = 2.0;
    let mut camera_focal_y = screen_height() / 2.0;
    let mut camera_focal_x = screen_width() / 2.0;
    let main_area_width = 570.;
    let mut camera_zoom : f32 = 10.0;

    let main_texture = load_texture("assets/vs-dx-atlas-padded.png").await.unwrap();
    main_texture.set_filter(FilterMode::Nearest);

    let mut player_pos_x = 64.;
    let mut player_pos_y = 64.;
    let mut player_flip_x: bool = false;

    let mut enemies: Vec<Enemies> = Vec::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let max_cooldown = Duration::from_secs(15).as_millis();
    let mut bullet_cooldown = max_cooldown;

    enemies.push(
        Enemies {
            position: Position {
                x: 72.,
                y: 90.,
            },
            collider: Collider { 
                x: 72., 
                y: 90., 
                width: 8, 
                height: 8,
                radius: 4.
            } 
        }
    );

    // enemies.push(
    //     Enemies {
    //         position: Position {
    //             x: 89.,
    //             y: 64.,
    //         },
    //         collider: Collider { 
    //             x: 72., 
    //             y: 90., 
    //             width: 8, 
    //             height: 8,
    //             radius: 4.
    //         } 
    //     }
    // );

    // enemies.push(
    //     Enemies {
    //         position: Position {
    //             x: 56.,
    //             y: 48.,
    //         },
    //         collider: Collider { 
    //             x: 72., 
    //             y: 90., 
    //             width: 8, 
    //             height: 8,
    //             radius: 4.
    //         } 
    //     }
    // );

    // enemies.push(
    //     Enemies {
    //         position: Position {
    //             x: 40.,
    //             y: 64.,
    //         },
    //         collider: Collider { 
    //             x: 72., 
    //             y: 90., 
    //             width: 8, 
    //             height: 8,
    //             radius: 4. 
    //         } 
    //     }
    // );

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

        move_player(&mut player_pos_x, &mut player_pos_y, &mut player_flip_x);

        for x in 0..80 {
            for y in 0..50 {
                draw_map_cell(main_texture, x, y);
            }
        }

        update_enemies_position(&mut enemies, &mut player_pos_x, &mut player_pos_y);
        update_enemies_pushing(&mut enemies);

        draw_player(main_texture, &mut player_pos_x, &mut player_pos_y, &player_flip_x);
        // draw_player_collider(&mut player_pos_x, &mut player_pos_y);

        draw_enemies(main_texture, &mut enemies, &mut player_pos_x, &mut player_pos_y);
        draw_enemies_collider(&mut enemies);

        draw_bullets(main_texture, &mut bullets);
        update_bullets(&mut bullets, &mut enemies);

        if bullet_cooldown <= 0 {
            spawn_bullet(&mut bullets, &mut enemies, &mut player_pos_x, &mut player_pos_y);
            bullet_cooldown = max_cooldown;
        } else {
            bullet_cooldown = bullet_cooldown.clamp(0, bullet_cooldown - 100);
        }

        // println!("{}", bullet_cooldown);

        bullets.retain(|b| b.active);

        set_default_camera();
        next_frame().await;
    }
}
