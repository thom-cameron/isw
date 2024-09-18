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
}

impl Stopwatch {
    pub fn update_time(&mut self) {
        self.current_time = self.start_time.elapsed();

        let interval_duration = self
            .get_current_interval()
            .map(|interval| interval.duration);
        if let Some(interval_duration) = interval_duration {
            self.interval_current_time = self.interval_start_time.elapsed();
            if self.interval_current_time >= interval_duration {
                self.intervals_elapsed += 1;
                if let Some(interval_list) = &self.interval_list {
                    if self.interval_i == interval_list.intervals.len() - 1 {
                        self.interval_i = 0;
                    } else {
                        self.interval_i += 1;
                    }
                }
            }
        }
    }

    pub fn get_current_interval(&self) -> Option<&Interval> {
        // Return the current interval based on the interval index
        self.interval_list
            .as_ref()
            .and_then(|interval_list| interval_list.intervals.get(self.interval_i))
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

        format!("interval time: {:02}:{:02}:{:02}.{} (remove this)", h, m, s, ms)
    }
}
