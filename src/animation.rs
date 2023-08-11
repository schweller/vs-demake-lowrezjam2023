use std::time::Duration;

use macroquad::prelude::*;

#[derive(Clone)]
pub struct Animation {
    pub frames: Vec<Rect>,
    pub frame_length: Vec<Duration>,
    pub anim_duration: Duration,
    pub current_frame_length: Duration,
    pub current_frame: usize,
    pub repeating: bool,
}

impl Animation {
    pub fn get_animation_source(&mut self, duration: Duration) -> Option<Rect> {
        self.anim_duration += duration;

        let frames_remaining = self.current_frame < self.frames.len() - 1;
        if frames_remaining || self.repeating {
            while self.anim_duration >= self.current_frame_length {
                self.current_frame = (self.current_frame + 1) % self.frames.len();
                self.anim_duration -= self.current_frame_length;
                self.current_frame_length = self.frame_length[self.current_frame];
            }
        } else if self.anim_duration > self.current_frame_length {
            self.anim_duration = self.current_frame_length
        }

        Some(self.frames[self.current_frame])
    }

    #[deprecated]
    fn is_animation_finished(&mut self) -> bool {
        if !self.repeating {
            return self.current_frame == self.frames.len() - 1
                || self.current_frame == 0
                || self.current_frame == (self.frames.len() - 1) / 2;
        }
        
        false
    }    
}