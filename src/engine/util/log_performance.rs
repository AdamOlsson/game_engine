use std::time::Instant;



pub struct LogPerformance {
    count: u32,
    timer: Instant 
}

impl LogPerformance {
    pub fn new() -> Self {
        Self { count: 0, timer: Instant::now() }
    }

    pub fn log(&mut self) {
        self.count += 1;
        let now = Instant::now();
        let diff = now.duration_since(self.timer);
        if diff.as_millis() > 1000 {
            let fps = self.count / diff.as_secs() as u32; 
            println!("fps: {}", fps);
            self.count = 0;
            self.timer = now;
        }
    }
}
