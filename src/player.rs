use macroquad::prelude::*;
use crate::direction::Direction;
use crate::timer::Timer;

use super::Position;

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
