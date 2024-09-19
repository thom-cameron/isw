use std::{
    fmt,
    time::{Duration, Instant},
};

use crate::intervals::{Interval, IntervalList};

#[derive(Debug)]
pub struct Stopwatch {
    pub start_time: Instant,
    pub current_time: Duration,
    display_time: DisplayTime,

    pub interval_start_time: Instant,
    pub interval_end_time: Instant,
    pub interval_remaining_time: Duration,
    pub interval_list: Option<IntervalList>,
    pub interval_i: usize,
    pub intervals_elapsed: usize,
    pub interval_cycles_elapsed: usize,

    pub paused: bool,
    pub paused_time_overall: Duration,
    pub paused_time_last: Duration,
    pub paused_start_time: Instant,
    pause_on_interval: bool,
}

impl Stopwatch {
    pub fn new(
        interval_list: Option<IntervalList>,
        count_down: bool,
        pause_on_interval: bool,
    ) -> Self {
        let start_time = Instant::now();
        let zero_duration = Duration::from_secs(0);
        let first_interval_end_time = match &interval_list {
            Some(interval_list) => start_time + interval_list.intervals[0].duration,
            None => start_time,
        };

        Self {
            start_time,
            current_time: zero_duration,
            display_time: if !count_down { DisplayTime::Current } else { DisplayTime::Countdown },

            interval_start_time: start_time,
            interval_end_time: first_interval_end_time,
            interval_remaining_time: zero_duration,
            interval_list,
            interval_i: 0,
            intervals_elapsed: 0,
            interval_cycles_elapsed: 0,

            paused: pause_on_interval,
            paused_time_overall: zero_duration,
            paused_time_last: zero_duration,
            paused_start_time: start_time,
            pause_on_interval,
        }
    }

    /// Update the current time on the stopwatch and reevaluate interval if
    /// applicable/necessary
    pub fn update_time(&mut self) {
        if !self.paused {
            self.current_time = self.start_time.elapsed() - self.paused_time_overall;

            if self.interval_list.is_some() {
                self.interval_remaining_time = self.interval_end_time - Instant::now();
                if self.interval_remaining_time <= Duration::from_secs(0) {
                    self.next_interval();
                }
            }
        }
    }

    fn next_interval(&mut self) {
        self.intervals_elapsed += 1;

        if let Some(interval_list) = &self.interval_list {
            if self.interval_i == interval_list.intervals.len() - 1 {
                self.interval_i = 0;
                self.interval_cycles_elapsed += 1;
            } else {
                self.interval_i += 1;
            }

            self.interval_start_time = self.interval_end_time;
            self.interval_end_time =
                self.interval_start_time + interval_list.intervals[self.interval_i].duration;
        }

        if self.pause_on_interval {
            self.toggle_pause();
        }
    }

    pub fn get_current_interval(&self) -> Option<&Interval> {
        self.interval_list
            .as_ref()
            .and_then(|interval_list| interval_list.intervals.get(self.interval_i))
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;

        match self.paused {
            true => self.paused_start_time = Instant::now(),
            false => {
                self.paused_time_last = self.paused_start_time.elapsed();
                self.paused_time_overall += self.paused_time_last;
                self.interval_end_time += self.paused_time_last;
            }
        }
    }

    fn get_formatted_time(&self) -> String {
        let display_time = match self.display_time {
            DisplayTime::Current => self.current_time,
            DisplayTime::Countdown => self.interval_remaining_time,
        };

        let total_s = display_time.as_secs();
        let h = total_s / 3600;
        let m = (total_s % 3600) / 60;
        let s = total_s % 60;
        let ms = display_time.subsec_millis() / 100;

        format!("{:02}:{:02}:{:02}.{}", h, m, s, ms)
    }
}

impl fmt::Display for Stopwatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_formatted_time())
    }
}

#[derive(Debug)]
enum DisplayTime {
    Current,
    Countdown,
}
