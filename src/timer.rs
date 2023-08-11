use std::time::Duration;
use std::ops::Sub;

#[derive(Debug, Clone, PartialEq)]
pub struct Timer{
	pub duration: Duration,
	start_time: instant::Instant,
}

impl Timer{
	pub fn new(duration_millis: u64)-> Timer{
		Timer{
			duration: Duration::from_millis(duration_millis),
			start_time: instant::Instant::now(),
		}
	}
	pub fn new_sec(duration_sec: u64)-> Timer{
		Timer{
			duration: Duration::from_secs(duration_sec),
			start_time: instant::Instant::now(),
		}
	}

	// pub fn advance_by(&mut self, duration: Duration){
	// 	self.start_time = self.start_time.sub(duration);
	// }

	pub fn finished(&self) -> bool{
		let current_time = instant::Instant::now();
		let elapsed = current_time - self.start_time;
		elapsed >= self.duration
	}

	pub fn set_duration_millis(&mut self, duration: u64){
		self.duration = Duration::from_millis(duration);
	}

	#[allow(dead_code)]
	pub fn set_duration(&mut self, duration: u64){
		self.duration = Duration::from_secs(duration);
	}

	pub fn restart(&mut self){
		self.start_time = instant::Instant::now();
	}

	// pub fn elapsed(&self) -> Duration {
	// 	let current_time = instant::Instant::now();
	// 	let elapsed = current_time - self.start_time;
	// 	elapsed	
	// }

	pub fn value(&self)-> f32{
		let current_time = instant::Instant::now();
		let elapsed = current_time - self.start_time;
		if elapsed < self.duration{
			1.0 * (100.0 / self.duration.as_millis() as f32 * elapsed.as_millis() as f32) / 100.0
		}else{
			1.0
		}
	}
}