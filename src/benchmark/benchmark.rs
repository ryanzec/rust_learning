use std::time::Instant;

pub struct Benchmark {
    start_time: Instant,
    end_time: Instant,
}

impl Benchmark {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            end_time: now,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn stop(&mut self) {
        self.end_time = Instant::now();
    }

    pub fn reset(&mut self) {
        self.start_time = Instant::now();
        self.end_time = Instant::now();
    }

    pub fn get_elapsed_time_message(&self, prepend_message: Option<&str>) -> String {
        match prepend_message {
            Some(message) => format!("{}: {:?}", message, self.end_time - self.start_time),
            None => format!("duration took: {:?}", self.end_time - self.start_time),
        }
    }
}
