use macroquad::prelude::*;
use tween::{Looper, Tweener, SineInOut, BounceInOut, Oscillator, CircInOut};

use crate::{timer::Timer, Upgrade, tween::Tween, TestTween};

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
}

pub fn draw_level_timer_ui(
    font: Font,
    mins: String,
    secs: String,
) {
    draw_text_ex(mins.as_str(), (screen_width() / 2.) - 80., 50., 
        TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
    );
    draw_text_ex(
        &(":".to_owned() + secs.as_str()), 
        ((screen_width() / 2.) - 80.) + 80., 50., 
    TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
)
}

pub fn draw_level_up_title(
    font: Font,
    tween: &mut TestTween<f32, f32>,
    letter_tween: &mut TestTween<f32, f32>
) {
    let delta = get_frame_time();
    draw_text_ex(
        "l",
        (screen_width() / 2.) - 150., 
        80. + letter_tween.move_by(delta), 
        TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
    );
    draw_text_ex(
        "e",
        (screen_width() / 2.) - 110., 
        80. - letter_tween.move_by(delta),
        TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
    );
    draw_text_ex(
        "v",
        (screen_width() / 2.) - 70., 
        80. + letter_tween.move_by(delta), 
        TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
    );
    draw_text_ex(
        "e",
        (screen_width() / 2.) - 30., 
        80. - letter_tween.move_by(delta), 
        TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
    );       
    draw_text_ex(
        "l",
        (screen_width() / 2.) + 10., 
        80. + letter_tween.move_by(delta), 
        TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
    );
    draw_text_ex(
        "u",
        (screen_width() / 2.) + 50., 
        80. - letter_tween.move_by(delta), 
        TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
    );
    draw_text_ex(
        "p",
        (screen_width() / 2.) + 90., 
        80. + letter_tween.move_by(delta), 
        TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
    );
    draw_text_ex(
        "!",
        (screen_width() / 2.) + 130., 
        80. - letter_tween.move_by(delta), 
        TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
    );

    draw_text_ex(
        "press Z to choose",
        (screen_width() / 2.) - 270., 
        150., 
        TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
    );

    // draw_text_ex(
    //     "LEVEL UP!",
    //     (screen_width() / 2.) - 150., 
    //     70. + tween.move_by(delta), 
    //     TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., ..Default::default()}
    // );
}

pub fn draw_level_up(
    choosen_upgrade_index: &i32,
    available_upgrades: &Vec<Box<dyn Upgrade>>,
    font: Font,
    tween: &mut Tween,
    init_tween: &mut TestTween<f32, f32>
) {
    // Level UP UI
    tween.update();
    draw_rectangle(0., 0., screen_width(), screen_height(), Color::new(0., 0., 0., 0.8));
    for (i, upgrade) in available_upgrades.iter().enumerate() {
        let delta = get_frame_time();
        let f = i as f32;
        let start = 120.;
        let total_spacing = 40.; // It will be the amount of upgrade available
        let upgrade_w = 180. + total_spacing;
        // let upgrade_h = 160.;
        // let mut x_pos = (screen_width() / 2.) - 245.;
        // let y_pos = start + f * (upgrade_h);
        let x_pos = start + f * (upgrade_w);
        let mut y_pos = screen_height() / 2.;
        // init_tween.move_by(delta);
        // if init_tween.is_finished() {
        // }
        if *choosen_upgrade_index == (i as i32) {
            y_pos -= 50. + tween.value();
        }
        upgrade.draw(
            font, 
            x_pos,
            y_pos, 
            *choosen_upgrade_index == (i as i32)
        );
    }
}

pub fn draw_stage_cleared() {
    
} 