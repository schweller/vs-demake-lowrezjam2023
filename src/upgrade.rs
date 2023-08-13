use macroquad::prelude::*;

pub trait Upgrade {
    fn get_name(&self) -> &'static str;
    fn draw(&self, font: Font, x: f32, y: f32, highlighted: bool);
    fn get_color(&self) -> Color {
        Color::from_hex(0x3e3546)
    }
}

pub struct SpeedUpgrade {}
pub struct FireRateUpgrade {}
pub struct RegenUpgrade {}

pub struct DashUpgrade {}
pub struct IframeUpgrade {}
pub struct PenShotUpgrade {}

fn draw_upgrade_bg(w: f32, h: f32, x: f32, y: f32) {
    draw_rectangle(
        x, 
        y-50., 
        w, 
        h, 
        Color::from_hex(0x905ea9)
    );
    draw_rectangle(
        (x+10.), 
        (y-10.)-50., 
        w-20., 
        h+20., 
        Color::from_hex(0x905ea9)
    );  
}

impl Upgrade for SpeedUpgrade {
    fn get_name(&self) -> &'static str {
        "Speed"
    }

    fn draw(&self, font: Font, x: f32, y: f32, highlighted: bool) {
        let mut font_color = self.get_color();
        if highlighted {
            font_color = WHITE;
        }
        draw_upgrade_bg(180., 180., x, y);
        draw_text_ex(
            "5%",
            x, 
            y, 
            TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., color: font_color, ..Default::default()}
        );
        draw_text_ex(
            "spd",
            x, 
            y+50., 
            TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., color: font_color, ..Default::default()}
        );
    }   
}

impl Upgrade for FireRateUpgrade {
    fn get_name(&self) -> &'static str {
        "FireRate"
    }
    fn draw(&self, font: Font, x: f32, y: f32, highlighted: bool) {
        let mut font_color = self.get_color();
        if highlighted {
            font_color = WHITE;
        }
        draw_upgrade_bg(180., 180., x, y);      
        draw_text_ex(
            "+1%",
            x, 
            y, 
            TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., color: font_color, ..Default::default()}
        );
        draw_text_ex(
            "Fire",
            x, 
            y+50., 
            TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., color: font_color, ..Default::default()}
        );
        draw_text_ex(
            "Rate",
            x, 
            y+100., 
            TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., color: font_color, ..Default::default()}
        );        
    }  
}

impl Upgrade for RegenUpgrade {
    fn get_name(&self) -> &'static str {
        "Recovery"
    }
    fn draw(&self, font: Font, x: f32, y: f32, highlighted: bool) {
        let mut font_color = self.get_color();
        if highlighted {
            font_color = WHITE;
        }
        draw_upgrade_bg(180., 180., x, y);  
        draw_text_ex(
            "+1%",
            x, 
            y, 
            TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., color: font_color, ..Default::default()}
        );
        draw_text_ex(
            "Regen",
            x, 
            y+50., 
            TextParams { font, font_size: 64, font_scale: 1., font_scale_aspect: 1., color: font_color, ..Default::default()}
        );
    }  
}

pub fn pick_random_upgrades() -> Vec<Box<dyn Upgrade>> {
    let mut upgrades : Vec<Box<dyn Upgrade>> = Vec::new();
    
    upgrades.push(Box::new(FireRateUpgrade{}));
    // upgrades.push(Box::new(SpeedUpgrade{}));
    upgrades.push(Box::new(RegenUpgrade{}));

    upgrades
}
