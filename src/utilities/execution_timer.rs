use std::time;

pub struct ExecutionTimer {
    last_executed: time::Instant,
    execution_interval: time::Duration,
}

impl ExecutionTimer {
    pub fn new(execution_interval: time::Duration) -> Self {
        ExecutionTimer {
            execution_interval,
            last_executed: time::Instant::now(),
        }
    }

    pub fn run<F>(&mut self, mut f: F)
    where
        F: FnMut(),
    {
        let now = time::Instant::now();
        if now.duration_since(self.last_executed) >= self.execution_interval {
            self.last_executed = now;

            f()
        }
    }
}
