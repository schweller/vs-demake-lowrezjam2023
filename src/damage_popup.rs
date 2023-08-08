use macroquad::prelude::*;
use keyframe::{Keyframe, functions::{EaseOut, EaseInOut}};

use crate::{tween::Tween, Position};

pub struct DamagePopup {
    pos: Position,
    tween: Tween,
    opacity_tween: Tween,
    pub active: bool
}

impl DamagePopup {
    pub fn new(x: f32, y: f32) -> Self {
        let opacity_tween = Tween::from_keyframes(
            vec![
                Keyframe::new(1.0, 0.0, EaseOut),
                Keyframe::new(0.0, 1.0, EaseOut),
            ],
            0,
            1,
            false,
        );
        let tween = Tween::from_keyframes(
            vec![
                Keyframe::new(0.0, 0.0, EaseOut),
                Keyframe::new(5.0, 0.2, EaseInOut),
            ],
            0,
            5,
            false,
        );
        DamagePopup { pos: Position { x, y }, tween, opacity_tween, active: true }
    }

    pub fn update(&mut self) {
        self.tween.update();
        self.opacity_tween.update();
        if self.tween.finished() {
            self.active = false;
        }
    }

    pub fn draw(&self, texture: Texture2D) {
        if self.active {
            draw_texture_ex(
                texture, 
                self.pos.x - 2., 
                (self.pos.y) - self.tween.value(), 
                Color::new(1., 1., 1., 0.5 * self.opacity_tween.value()), 
                DrawTextureParams { 
                    dest_size: Some(vec2(6., 4.)),
                    source: Some(Rect::new(
                        0.,
                        8.,
                        6.,
                        4.,
                    )),
                    ..Default::default()
                }
            );
        }
    }
}
