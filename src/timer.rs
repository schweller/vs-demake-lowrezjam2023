use std::time::Duration;
use macroquad::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Timer{
	pub duration: Duration,
	start_time: f64,
}

impl Timer{
	pub fn new(duration_millis: u64)-> Timer{
		Timer{
			duration: Duration::from_millis(duration_millis),
			start_time: miniquad::date::now(),
		}
	}

	pub fn new_sec(duration_sec: u64)-> Timer{
		Timer{
			duration: Duration::from_secs(duration_sec),
			start_time: miniquad::date::now(),
		}
	}

	pub fn finished(&self) -> bool{
		let current_time = miniquad::date::now();
		let elapsed = current_time - self.start_time;
		elapsed >= self.duration.as_secs_f64()
	}

	pub fn set_duration_millis(&mut self, duration: u64){
		self.duration = Duration::from_millis(duration);
	}

	#[allow(dead_code)]
	pub fn set_duration(&mut self, duration: u64){
		self.duration = Duration::from_secs(duration);
	}

	pub fn restart(&mut self){
		self.start_time = miniquad::date::now();
	}

	pub fn elapsed(&self) -> Duration {
		let current_time = miniquad::date::now();
		let elapsed = current_time - self.start_time;
		Duration::from_secs_f64(elapsed)
	}

	pub fn value(&self)-> f32{
		let current_time = miniquad::date::now();
		let elapsed = current_time - self.start_time;
		if elapsed < self.duration.as_secs_f64(){
			1.0 * (100.0 / self.duration.as_millis() as f32 * elapsed as f32) / 100.0
		}else{
			1.0
		}
	}
}