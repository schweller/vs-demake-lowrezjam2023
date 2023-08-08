use macroquad::prelude::*;

pub trait Upgrade {
    fn get_name(&self) -> &'static str;
    fn render(&self, texture: Texture2D, x: f32, y: f32, highlighted: bool);
}

pub struct SpeedUpgrade {}
pub struct FireRateUpgrade {}
pub struct RecoveryUpgrade {}

impl Upgrade for SpeedUpgrade {
    fn get_name(&self) -> &'static str {
        "Speed"
    }

    fn render(&self, texture: Texture2D, x: f32, y: f32, highlighted: bool) {
        let mut source = Some(Rect::new(
            0.,
            0.,
            54.,
            16.,
        ));
        if highlighted {
            source = Some(Rect::new(
                54.,
                0.,
                54.,
                16.,
            ))
        }
        draw_texture_ex(
            texture, x, y, WHITE,
            DrawTextureParams { 
                dest_size: Some(vec2(54. * 10., 16. * 10.)),
                source,
                ..Default::default()
            }
        )
    }
}

impl Upgrade for FireRateUpgrade {
    fn get_name(&self) -> &'static str {
        "FireRate"
    }
    fn render(&self, texture: Texture2D, x: f32, y: f32, highlighted: bool) {
        let mut source = Some(Rect::new(
            0.,
            32.,
            54.,
            16.,
        ));
        if highlighted {
            source = Some(Rect::new(
                54.,
                32.,
                54.,
                16.,
            ))
        }
        draw_texture_ex(
            texture, x, y, WHITE,
            DrawTextureParams { 
                dest_size: Some(vec2(54. * 10., 16. * 10.)),
                source,
                ..Default::default()
            }
        )
    }  
}

impl Upgrade for RecoveryUpgrade {
    fn get_name(&self) -> &'static str {
        "Recovery"
    }
    fn render(&self, texture: Texture2D, x: f32, y: f32, highlighted: bool) {
        let mut source = Some(Rect::new(
            0.,
            16.,
            54.,
            16.,
        ));
        if highlighted {
            source = Some(Rect::new(
                54.,
                16.,
                54.,
                16.,
            ))
        }        
        draw_texture_ex(
            texture, x, y, WHITE,
            DrawTextureParams { 
                dest_size: Some(vec2(54. * 10., 16. * 10.)),
                source,
                ..Default::default()
            }
        )
    }  
}

pub fn pick_random_upgrades() -> Vec<Box<dyn Upgrade>> {
    let mut upgrades : Vec<Box<dyn Upgrade>> = Vec::new();
    
    upgrades.push(Box::new(FireRateUpgrade{}));
    upgrades.push(Box::new(SpeedUpgrade{}));
    upgrades.push(Box::new(RecoveryUpgrade{}));

    upgrades
}
