use macroquad::prelude::*;
use specs::*;

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, data: Self::SystemData) {
        let position = data;
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

fn main_loop(world: &mut World) {
    let mut hello_world = HelloWorld;
    hello_world.run_now(&world);
    world.maintain();
}

fn window_conf() -> Conf {
    Conf { 
        window_title: "Rustlike".to_owned(), 
        window_width: 640, // 640 + 120 
        window_height: 640, // 320 + 120
        ..Default::default()
    }
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

fn draw_player(texture: Texture2D, x: &mut f32, y: &mut f32) {
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
        ..Default::default()
    })
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

const PLAYER_SPEED: f32 = 20.;

fn move_player(x: &mut f32, y: &mut f32, ) {
    let delta = get_frame_time();
    if is_key_down(KeyCode::Left) {
        *x -= PLAYER_SPEED * delta;
    }
    if is_key_down(KeyCode::Right) {
        *x += PLAYER_SPEED * delta;
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
    println!("Hello, world!");
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

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

    let map = vec![true; 80*50];

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

        move_player(&mut player_pos_x, &mut player_pos_y);

        for x in 0..80 {
            for y in 0..50 {
                draw_map_cell(main_texture, x, y);
            }
        }

        draw_player(main_texture, &mut player_pos_x, &mut player_pos_y);

        draw_texture_ex(
            main_texture, 
            72.,
            90., 
            WHITE,
    DrawTextureParams { 
                dest_size: Some(vec2(8., 8.)), 
                source: Some(Rect::new(
                    20.,
                    2.,
                    8.,
                    8.,
                )),
                flip_x: true,
            ..Default::default()
        });

        draw_texture_ex(
            main_texture, 
            89.,
            64., 
            WHITE,
    DrawTextureParams { 
                dest_size: Some(vec2(8., 8.)), 
                source: Some(Rect::new(
                    20.,
                    2.,
                    8.,
                    8.,
                )),
                flip_x: true,
            ..Default::default()
        });      

        draw_texture_ex(
            main_texture, 
            56.,
            48., 
            WHITE,
    DrawTextureParams { 
                dest_size: Some(vec2(8., 8.)), 
                source: Some(Rect::new(
                    20.,
                    2.,
                    8.,
                    8.,
                )),
                flip_x: false,
            ..Default::default()
        });

        draw_texture_ex(
            main_texture, 
            40.,
            60., 
            WHITE,
    DrawTextureParams { 
                dest_size: Some(vec2(8., 8.)), 
                source: Some(Rect::new(
                    20.,
                    2.,
                    8.,
                    8.,
                )),
                flip_x: false,
            ..Default::default()
        });                  

        main_loop(&mut world);
        set_default_camera();
        next_frame().await;
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}