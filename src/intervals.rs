use std::io;
use std::str::FromStr;
use std::time::Duration;

use ratatui::style::Color;

const MAX_COLOUR_N: u8 = 7;

#[derive(Debug)]
pub struct IntervalList {
    pub intervals: Vec<Interval>,
}

impl IntervalList {
    pub fn new(
        durations_string: String,
        optional_colours_string: Option<String>,
    ) -> Result<Self, io::Error> {
        let durations = Self::parse_int_string::<u32>(durations_string)?;

        let optional_colours: Option<Vec<u8>> = match optional_colours_string {
            Some(colours_string) => Some(Self::parse_int_string::<u8>(colours_string)?),
            None => None,
        };
        let colours = Self::determine_colours(optional_colours, &durations);

        let intervals = (0..(durations.len()))
            .map(|i| Interval::new(durations[i], colours[i]))
            .collect();

        Ok(Self { intervals })
    }

    /// parse comma separated strings of integers from the cli
    fn parse_int_string<T>(int_string: String) -> Result<Vec<T>, io::Error>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        let ints = int_string
            .split(',')
            .map(str::parse::<T>)
            .collect::<Result<Vec<T>, T::Err>>()
            .map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse string: {}", e))
            })?;
        Ok(ints)
    }

    /// determine which colours to use for intervals
    fn determine_colours(optional_colours: Option<Vec<u8>>, durations: &[u32]) -> Vec<u8> {
        // generate colours if none are provided
        let colours = match optional_colours {
            Some(colours) => colours,
            None => return Self::generate_colours(durations.len()),
        };

        // generate colours if the wrong number are provided
        if colours.len() != durations.len() {
            eprintln!(
                "Different number of interval durations and colours provided. Colours will be \
                 automatically generated."
            );
            return Self::generate_colours(durations.len());
        }

        // generate colours if invalid codes are provided
        let max_colour = match colours.iter().max() {
            Some(&max) => max,
            None => {
                eprintln!(
                    "Error evaluating the colours provided. Colours will be automatically \
                     generated."
                );
                return Self::generate_colours(durations.len());
            }
        };
        if max_colour > MAX_COLOUR_N {
            eprintln!(
                "Invalid ANSI colour code provided. Colours will be automatically generated."
            );
            return Self::generate_colours(durations.len());
        }

        // use the provided colours if they are valid
        colours
    }

    // generate a colour for each interval by cycling through the ANSI options
    fn generate_colours(n_colours: usize) -> Vec<u8> {
        (0..n_colours)
            .map(|n| (n as u8 % MAX_COLOUR_N) + 1)
            .collect()
    }
}

#[derive(Debug)]
pub struct Interval {
    pub duration: Duration,
    pub colour: Color,
}

impl Interval {
    pub fn new(duration_s: u32, colour: u8) -> Self {
        Self {
            duration: Duration::from_secs(duration_s.into()),
            colour: match colour {
                0 => Color::Black,
                1 => Color::Red,
                2 => Color::Green,
                3 => Color::Yellow,
                4 => Color::Blue,
                5 => Color::Magenta,
                6 => Color::Cyan,
                7 => Color::White,
                _ => Color::White,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int_vec_to_string<T>(vec: Vec<T>) -> String
    where
        T: FromStr + ToString,
        T::Err: std::fmt::Display,
    {
        vec.iter()
            .map(|duration| duration.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    #[test]
    fn valid_configurations() {
        for (durations, colours) in [
            (vec![60], None),
            (vec![30; 100], None),
            (vec![60], Some(vec![7])),
            (vec![30, 30, 30], Some(vec![1, 2, 3])),
        ] {
            // construct a list of intervals from the raw values
            let colours = match colours {
                Some(colours) => colours,
                None => IntervalList::generate_colours(durations.len()),
            };
            let raw_interval_list: Vec<Interval> = durations
                .iter()
                .zip(colours.iter())
                .map(|(&duration, &colour)| Interval::new(duration, colour))
                .collect();

            // convert the raw values to strings and construct an IntervalList
            let parsed_interval_list =
                IntervalList::new(int_vec_to_string(durations), Some(int_vec_to_string(colours)))
                    .expect("Failed to parse the test interval configuration strings");

            // compare the resulting Interval instances
            for (raw_interval, parsed_interval) in raw_interval_list
                .iter()
                .zip(parsed_interval_list.intervals.iter())
            {
                assert_eq!(raw_interval.duration, parsed_interval.duration);
                assert_eq!(raw_interval.colour, parsed_interval.colour);
            }
        }
    }

    #[test]
    fn invalid_configurations() {
        for (durations_string, colours_string) in [
            (",".to_string(), None),
            ("30,".to_string(), None),
            ("30".to_string(), Some(",".to_string())),
            ("30".to_string(), Some("1,".to_string())),
        ] {
            let result = IntervalList::new(durations_string, colours_string);

            assert!(result.is_err());
        }
    }

    #[test]
    fn generate_colours() {
        for (durations, correct_colours) in
            [(vec![60], vec![1]), (vec![30; 8], vec![1, 2, 3, 4, 5, 6, 7, 1])]
        {
            let generated_colours = IntervalList::generate_colours(durations.len());

            assert_eq!(correct_colours, generated_colours);
        }
    }
}
