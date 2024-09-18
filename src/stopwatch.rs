use std::time::{Duration, Instant};

use crate::intervals::{Interval, IntervalList};

#[derive(Debug)]
pub struct Stopwatch {
    pub start_time: Instant,
    pub current_time: Duration,
    pub interval_start_time: Instant,
    pub interval_current_time: Duration,
    pub interval_list: Option<IntervalList>,
    pub interval_i: usize,
    pub intervals_elapsed: usize,
    pub interval_cycles_elapsed: usize,
}

impl Stopwatch {
    pub fn new(interval_list: Option<IntervalList>) -> Self {
        let start_time = Instant::now();
        let zero_duration = Duration::new(0, 0);

        Self {
            start_time,
            current_time: zero_duration,
            interval_start_time: start_time,
            interval_current_time: zero_duration,
            interval_list,
            interval_i: 0,
            intervals_elapsed: 0,
            interval_cycles_elapsed: 0,
        }
    }

    pub fn update_time(&mut self) {
        self.current_time = self.start_time.elapsed();

        let interval_duration = self
            .get_current_interval()
            .map(|interval| interval.duration);
        if let Some(interval_duration) = interval_duration {
            self.interval_current_time = self.interval_start_time.elapsed();
            if self.interval_current_time >= interval_duration {
                self.change_interval();
            }
        }
    }

    pub fn get_current_interval(&self) -> Option<&Interval> {
        self.interval_list
            .as_ref()
            .and_then(|interval_list| interval_list.intervals.get(self.interval_i))
    }

    fn change_interval(&mut self) {
        self.interval_start_time = Instant::now();
        self.interval_current_time = Duration::new(0, 0);

        self.intervals_elapsed += 1;

        if let Some(interval_list) = &self.interval_list {
            if self.interval_i == interval_list.intervals.len() - 1 {
                self.interval_i = 0;
                self.interval_cycles_elapsed += 1;
            } else {
                self.interval_i += 1;
            }
        }
    }

    pub fn get_formatted_time(&self) -> String {
        let total_s = self.current_time.as_secs();

        let h = total_s / 3600;
        let m = (total_s % 3600) / 60;
        let s = total_s % 60;
        let ms = self.current_time.subsec_millis() / 100;

        format!("{:02}:{:02}:{:02}.{}", h, m, s, ms)
    }
    pub fn get_formatted_interval_time(&self) -> String {
        let total_s = self.interval_current_time.as_secs();

        let h = total_s / 3600;
        let m = (total_s % 3600) / 60;
        let s = total_s % 60;
        let ms = self.interval_current_time.subsec_millis() / 100;

        format!(
            "interval time: {:02}:{:02}:{:02}.{} (remove this)",
            h, m, s, ms
        )
    }
}
