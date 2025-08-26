use ark_std::time::{Duration, Instant};

pub struct Timer {
    timestamp: Instant,
    duration: Duration,
    is_running: bool,
}

impl Timer {
    fn new() -> Self {
        Timer {
            timestamp: Instant::now(),
            duration: Duration::ZERO,
            is_running: false,
        }
    }

    pub fn start(&mut self) {
        if self.is_running {
            panic!("Cannot start already running timer")
        }
        self.is_running = true;
        self.timestamp = Instant::now();
    }

    pub fn stop(&mut self) {
        let time = self.timestamp.elapsed();
        self.duration += time;
        self.is_running = false;
    }

    fn get_nanos(self) -> u128 {
        if self.is_running {
            panic!("Timer is still running")
        }
        self.duration.as_nanos()
    }
}

pub struct TimingData {
    data: Vec<u128>,
}

impl TimingData {
    pub fn analyse(&self) -> (f64, f64, f64) {
        let mut min = u128::MAX;
        let mut max = u128::MIN;
        let mut total = 0;
        for &t in &self.data {
            if t < min {
                min = t;
            }
            if t > max {
                max = t;
            }
            total += t;
        }
        let mean = total / (self.data.len() as u128);
        (min as f64 / 1e9, max as f64 / 1e9, mean as f64 / 1e9)
    }
}

pub fn benchmark<T>(n: usize, arg: T, target: fn(&T, &mut Timer)) -> TimingData {
    let mut data = Vec::new();
    for _ in 0..n {
        let mut timer = Timer::new();
        target(&arg, &mut timer);
        let time = timer.get_nanos();
        data.push(time);
    }
    TimingData { data }
}
