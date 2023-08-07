use macroquad::prelude::*;

use crate::{timer::Timer, Upgrade};

pub fn draw_level_ui(
    texture: Texture2D,
    current_player_hp_percentage: &f32,
    current_player_xp_percentage: &f32,
    player_level: &i32,
    player_inv_timer: &Timer
) {
    draw_rectangle(0., screen_height() - 80., screen_width(), 120., BLACK);
    let zoom = 10.;
    // HP
    let hp_bar_height = 20.;
    draw_rectangle(
        90.,
        screen_height() - hp_bar_height - 40., 
        ((screen_width() - 90.)*current_player_hp_percentage)/100., 
        hp_bar_height, 
        Color::from_hex(0xf04f78)
    );
    draw_texture_ex(
        texture, 
        10., 
        screen_height() - hp_bar_height - 50., 
        WHITE,
        DrawTextureParams { 
            dest_size: Some(vec2(8. * zoom, 3. * zoom)),
            source: Some(Rect::new(
                0.,
                0.,
                8.,
                3.,
            )),
            ..Default::default()
        } 
    );
    // XP
    let xp_bar_height = 20.;
    draw_rectangle(
        90.,
        screen_height() - xp_bar_height,  
        ((screen_width() - 90.)*current_player_xp_percentage)/100., 
        xp_bar_height, 
        Color::from_hex(0x4d65b4)
    );
    draw_texture_ex(
        texture, 
        10., 
        screen_height() - xp_bar_height - 10., 
        WHITE,
        DrawTextureParams { 
            dest_size: Some(vec2(8. * zoom, 3. * zoom)),
            source: Some(Rect::new(
                0.,
                4.,
                8.,
                3.,
            )),
            ..Default::default()
        } 
    );

    //Player level
    // draw_text(format!("Level {}", player_level).as_str(), 10., 40., 50., WHITE);
    // draw_text(format!("Speed {}", player_speed_bonus).as_str(), 10., 80., 50., WHITE);                           
    // draw_text(format!("Inv {}", player_inv_timer.value()).as_str(), 10., 80., 50., WHITE);
}

pub fn draw_level_up(
    choosen_upgrade_index: &i32,
    ll: &Vec<Box<dyn Upgrade>>
) {
    // Level UP UI
    draw_rectangle(0., 0., screen_width(), screen_height(), Color::new(0., 0., 0., 0.5));
    draw_text("LEVEL UP!", screen_width()/2. - 50., 100., 30., WHITE);

    // Choices
    // draw three rectangles
    // input chooses between index
    // highlight indexed rectangle

    for i in 0..3 {
        let f = i as f32;
        let start = 10.;
        let total_spacing = 40.; // It will be the amount of upgrade available
        let upgrade_w = (screen_width() - total_spacing) / 3.;
        let upgrade_h = screen_height() / 3.;
        let x_pos = start + f * (upgrade_w + 10.);
        let mut color = Color::new(0., 255., 0., 1.);
        if *choosen_upgrade_index == i {
            color = Color::new(255., 0., 0., 1.);
        }
        draw_rectangle(
            x_pos,
            screen_height() / 2. - ((screen_height() / 3.) / 2.), 
            upgrade_w, 
            upgrade_h, 
            color
        );
    }
}