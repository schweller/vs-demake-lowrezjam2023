use instant::Instant;
use std::time::Duration;
use macroquad::prelude::*;

pub fn now() -> f64 {
    miniquad::date::now()
    // std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH)
    //                             .expect("System clock was before 1970.")
    //                             .as_secs_f64() * 1000.0
}

enum State {
    Running {
        lap_start_time: Instant,
        lap_start_time_p: f64
    },
    Stopped {
        lap_start_time: Instant,
        lap_start_time_p: f64,
        suspend_time: Instant,
        suspend_time_p: f64
    },
}
use State::*;

/// The stopwatch.
pub struct StopWatch {
    start_time: Instant,
    start_time_patch: f64,
    state: State,
    cur_suspend: Duration,
    total_suspend: Duration,
}
impl StopWatch {
    /// Start a stopwatch.
    ///
    /// ```
    /// use std::time::Duration;
    /// use std::thread::sleep;
    ///
    /// let mut sw = stopwatch_rs::StopWatch::start();
    /// sleep(Duration::from_secs(1));
    /// let sp1 = sw.split(); // split=1s, lap=1s
    /// sw.suspend();
    /// sleep(Duration::from_secs(2));
    /// sw.resume();
    /// sleep(Duration::from_secs(4));
    /// let sp2 = sw.split(); // split=5s, lap=4s
    /// ```
    pub fn start() -> Self {
        let now = Instant::now();
        let now_p = miniquad::date::now();
        Self {
            start_time: now,
            start_time_patch: now_p,
            state: Running {
                lap_start_time: now,
                lap_start_time_p: now_p,
            },
            cur_suspend: Duration::new(0, 0),
            total_suspend: Duration::new(0, 0),
        }
    }
    /// Temporarily suspend the stopwatch. The clock is suspended until it is resumed.
    pub fn suspend(&mut self) {
        if let Running {
            lap_start_time: start_time,
            lap_start_time_p: start_time_patch,
        } = self.state
        {
            let now = Instant::now();
            let now_p = miniquad::date::now();
            self.state = Stopped {
                lap_start_time: start_time,
                lap_start_time_p: start_time_patch,
                suspend_time: now,
                suspend_time_p: now_p
            };
        }
    }
    /// Resume the stopwatch.
    pub fn resume(&mut self) {
        if let Stopped {
            lap_start_time: start_time,
            lap_start_time_p: start_time_patch,
            suspend_time,
            suspend_time_p
        } = self.state
        {
            let now = Instant::now();
            let now_p = miniquad::date::now();
            let suspend_time_p = Duration::new((now_p - suspend_time_p) as u64, 0);
            let suspend_time = now.duration_since(suspend_time);
            self.cur_suspend += suspend_time_p;
            self.total_suspend += suspend_time_p;
            self.state = Running {
                lap_start_time: start_time,
                lap_start_time_p: start_time_patch,
            }
        }
    }
    /// Consume the current state and return the split time and the lap time.
    pub fn split(&mut self) -> Split {
        match self.state {
            State::Running {
                lap_start_time: start_time,
                lap_start_time_p: start_time_patch,
            } => {
                let now = Instant::now();
                let now_p = miniquad::date::now();
                // let lap_p = now_p - start_time_patch - Duration::as_secs_f64(&self.cur_suspend);
                let lap_p = Duration::new((now_p - start_time_patch) as u64, 0) - self.cur_suspend;
                let lap = now.duration_since(start_time) - self.cur_suspend;
                let split_p = Duration::new((now_p - start_time_patch) as u64, 0) - self.total_suspend;
                // let split_p = now_p - start_time_patch - Duration::as_secs_f64(&self.total_suspend);
                let split = now.duration_since(self.start_time) - self.total_suspend;
                self.state = Running {
                    lap_start_time: now,
                    lap_start_time_p: now_p,
                };
                self.cur_suspend = Duration::new(0, 0);
                println!("{} {}", lap.as_millis(), split.as_millis());
                Split { split: split_p, lap: lap_p }
            }
            State::Stopped {
                lap_start_time: start_time,
                lap_start_time_p: start_time_patch,
                suspend_time,
                suspend_time_p
            } => {
                let lap_p = suspend_time_p - start_time_patch - Duration::as_secs_f64(&self.cur_suspend);
                let lap = suspend_time.duration_since(start_time) - self.cur_suspend;
                let split_p = suspend_time_p - start_time_patch - Duration::as_secs_f64(&self.total_suspend);
                let split = suspend_time.duration_since(self.start_time) - self.total_suspend;
                Split { split: Duration::from_secs_f64(split_p), lap: Duration::from_secs_f64(lap_p) }
            }
        }
    }
}
pub struct Split {
    /// Time spent after the stopwatch's started.
    pub split: Duration,
    /// Time spent between two splits.
    pub lap: Duration,
}
impl std::fmt::Display for Split {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "split={:?}, lap={:?}", self.split, self.lap)
    }
}