use std::{
    fmt,
    time::{Duration, Instant},
};

use crate::intervals::{Interval, IntervalList};

#[derive(Debug)]
pub struct Stopwatch {
    start_time: Instant,
    pub current_time: Duration,
    display_time: DisplayTime,

    interval_start_time: Instant,
    interval_end_time: Instant,
    interval_remaining_time: Duration,
    interval_list: Option<IntervalList>,
    interval_i: usize,
    intervals_elapsed: usize,
    interval_cycles_elapsed: usize,
    show_interval: bool,
    show_cycle: bool,

    paused: bool,
    paused_time_overall: Duration,
    paused_time_last: Duration,
    paused_start_time: Instant,
    pause_on_interval: bool,

    interval_shell_command: Option<String>,
}

impl Stopwatch {
    pub fn new(
        interval_list: Option<IntervalList>,
        count_down: bool,
        pause_on_interval: bool,
        shell_command: Option<String>,
        show_interval: bool,
        show_cycle: bool,
    ) -> Self {
        let (start_time, zero_duration, first_interval_end_time) =
            Self::initialize_timers(&interval_list);

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
            show_interval,
            show_cycle,

            paused: pause_on_interval,
            paused_time_overall: zero_duration,
            paused_time_last: zero_duration,
            paused_start_time: start_time,
            pause_on_interval,

            interval_shell_command: shell_command,
        }
    }

    fn initialize_timers(interval_list: &Option<IntervalList>) -> (Instant, Duration, Instant) {
        let start_time = Instant::now();
        let zero_duration = Duration::from_secs(0);
        let first_interval_end_time = match interval_list {
            Some(interval_list) => start_time + interval_list.intervals[0].duration,
            None => start_time,
        };
        (start_time, zero_duration, first_interval_end_time)
    }

    /// update the current time on the stopwatch and re-evaluate interval if
    /// applicable/necessary
    pub fn update_time(&mut self) {
        if !self.paused {
            self.current_time = self.start_time.elapsed() - self.paused_time_overall;

            if self.interval_list.is_some() {
                self.interval_remaining_time = self.interval_end_time - Instant::now();
                if self.interval_remaining_time <= Duration::from_secs(0) {
                    self.interval_boundary();
                }
            }
        }
    }

    /// execute actions when an interval boundary is reached
    fn interval_boundary(&mut self) {
        self.next_interval();

        self.execute_shell_command();

        if self.pause_on_interval {
            self.toggle_pause();
        }
    }

    /// move to the next interval
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
    }

    /// run the shell command provided by the user in the cli
    fn execute_shell_command(&mut self) {
        if let Some(shell_command) = &self.interval_shell_command {
            let shell_command = &shell_command
                .replace("%i", &self.intervals_elapsed.to_string())
                .replace("%c", &self.interval_cycles_elapsed.to_string());

            let mut arguments = shell_command.split_whitespace();
            if let Some(command) = arguments.next() {
                let _ = std::process::Command::new(command).args(arguments).spawn();
            };
        };
    }

    /// toggle the pause state and track how long pauses last
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;

        match self.paused {
            // pausing
            true => self.paused_start_time = Instant::now(),
            // unpausing
            false => {
                self.paused_time_last = self.paused_start_time.elapsed();
                self.paused_time_overall += self.paused_time_last;
                self.interval_end_time += self.paused_time_last;
            }
        }
    }

    /// get the current interval from the interval list
    pub fn get_current_interval(&self) -> Option<&Interval> {
        self.interval_list
            .as_ref()
            .and_then(|interval_list| interval_list.intervals.get(self.interval_i))
    }

    /// create the status string to display under the stopwatch
    pub fn get_status_string(&self) -> String {
        let mut status_string_parts = Vec::new();

        if self.show_interval {
            status_string_parts.push(format!("i{}", self.intervals_elapsed + 1));
        };
        if self.show_cycle {
            status_string_parts.push(format!("c{}", self.interval_cycles_elapsed + 1));
        };
        if self.paused {
            status_string_parts.push("•".to_string());
        };

        status_string_parts.join(" ")
    }

    /// reset the stopwatch
    pub fn reset(&mut self) {
        let (start_time, zero_duration, first_interval_end_time) =
            Self::initialize_timers(&self.interval_list);

        self.start_time = start_time;
        self.current_time = zero_duration;

        self.interval_start_time = start_time;
        self.interval_end_time = first_interval_end_time;
        self.interval_remaining_time = zero_duration;
        self.interval_i = 0;
        self.intervals_elapsed = 0;
        self.interval_cycles_elapsed = 0;

        self.paused_time_overall = zero_duration;
        self.paused_time_last = zero_duration;
        self.paused_start_time = start_time;
    }
}

impl fmt::Display for Stopwatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_time = match self.display_time {
            DisplayTime::Current => self.current_time,
            DisplayTime::Countdown => self.interval_remaining_time,
        };

        let total_s = display_time.as_secs();
        let h = total_s / 3600;
        let m = (total_s % 3600) / 60;
        let s = total_s % 60;
        let ms = display_time.subsec_millis() / 100;

        write!(f, "{:02}:{:02}:{:02}.{}", h, m, s, ms)
    }
}

#[derive(Debug)]
enum DisplayTime {
    Current,
    Countdown,
}
