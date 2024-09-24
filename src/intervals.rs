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
        intervals_string: String,
        optional_colours_string: Option<String>,
    ) -> Result<Self, io::Error> {
        let durations = Self::parse_int_string::<u32>(intervals_string)?;

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
