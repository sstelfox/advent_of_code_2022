use std::ops::Range;

use lazy_static::lazy_static;
use regex::Regex;

const INPUT_DATA: &'static [u8] = include_bytes!("../data/input");

lazy_static! {
    static ref LINE_MATCH: Regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
}

#[derive(Debug)]
struct Environment {
    sensors: Vec<Sensor>,
}

impl Environment {
    /// Automatically create a possible bounding box for the entire map's visibility based on the
    /// sensor's detection range.
    #[cfg(test)]
    fn aabb(&self) -> (isize, isize, isize, isize) {
        (
            self.sensors
                .iter()
                .map(|s| s.min_x_visible())
                .min()
                .unwrap(),
            self.sensors
                .iter()
                .map(|s| s.min_y_visible())
                .min()
                .unwrap(),
            self.sensors
                .iter()
                .map(|s| s.max_x_visible())
                .max()
                .unwrap(),
            self.sensors
                .iter()
                .map(|s| s.max_y_visible())
                .max()
                .unwrap(),
        )
    }

    fn detectable_positions_within_row(&self, row_coord: isize) -> usize {
        let mut detectable_positions = 0;

        for col_coord in self.relevant_row_range(row_coord) {
            let tgt_coord = (col_coord, row_coord);

            if self
                .sensors_within_range_of_row(row_coord)
                .any(|s| s.detectable_and_empty(tgt_coord))
            {
                detectable_positions += 1;
            }
        }

        detectable_positions
    }

    fn new(sensors: Vec<Sensor>) -> Environment {
        Self { sensors }
    }

    /// This is not a precise method, it is intended to help scope down the range of the X
    /// coordinate to limit the number of spaces that need to be checked. This returns the range of
    /// minimum and maximum X coordinates that any sensor able to see a particular row could
    /// possibly see.
    ///
    /// If the sensor does not exist on the same row as the provided coordinate it, it will not
    /// actually be able to see the minimum and maximum x coordinate returned.
    fn relevant_row_range(&self, row_coord: isize) -> Range<isize> {
        let min_x = self
            .sensors_within_range_of_row(row_coord)
            .map(|s| s.min_x_visible())
            .min()
            .unwrap();
        let max_x = self
            .sensors_within_range_of_row(row_coord)
            .map(|s| s.max_x_visible())
            .max()
            .unwrap();

        Range {
            start: min_x,
            end: max_x,
        }
    }

    fn search_within_bounds(&self, bounds: (isize, isize, isize, isize)) -> Option<(isize, isize)> {
        let minimum = (bounds.0, bounds.1);
        let maximum = (bounds.2, bounds.3);

        for search_y in (minimum.1)..=(maximum.1) {
            let relevant_sensors = self.sensors_within_range_of_row(search_y);

            // We know the sensor is within the bounds of a detection range on our row, so
            // constrain our search to the min / max on the row.
            let min_detectable_x = relevant_sensors.clone().map(|s| s.min_x_visible()).min().unwrap();
            let max_detectable_x = relevant_sensors.clone().map(|s| s.max_x_visible()).max().unwrap();

            let mut search_x = minimum.0.max(min_detectable_x);
            let max_search_x = maximum.0.min(max_detectable_x);

            loop {
                if search_x > max_search_x {
                    break;
                }

                // Multiple sensors may be detecting the same location
                let specific_detecting_sensors: Vec<&Sensor> = relevant_sensors
                    .clone()
                    .filter(|s| s.within_detection_range((search_x, search_y)))
                    .collect();

                if specific_detecting_sensors.is_empty() {
                    // No sensors are able to detect this location within our bounds
                    return Some((search_x, search_y));
                }

                // One or more sensors are able to see this location, we can skip a chunk of
                // evaluation by figuring out the furthest X coordinate any of this group of
                // sensors can see and immediately skipping to it
                //let max_real_detectable = specific_detecting_sensors
                //    .iter()
                //    .map(|s| s.max_x_visible_on_row(search_y))
                //    .max()
                //    .unwrap();

                //search_x = max_real_detectable + 1;
                search_x += 1;
            }
        }

        None
    }

    fn sensors_within_range_of_row<'a>(
        &'a self,
        row_coord: isize,
    ) -> impl Clone + Iterator<Item = &'a Sensor> {
        self.sensors
            .iter()
            .filter(move |s| s.can_detect_row(row_coord))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Sensor {
    location: (isize, isize),

    detected_beacon: (isize, isize),
    beacon_distance: usize,
}

impl Sensor {
    /// The absolute minimum y distance between two points is when they are sharing an x
    /// coordinate. This method is used to find only the sensors that are capable of seeing at
    /// least one location in a row.
    fn can_detect_row(&self, row_coord: isize) -> bool {
        self.within_detection_range((self.location.0, row_coord))
    }

    /// It's not enough to know the whether a position is detectable by a sensor, we also need to
    /// remove the locations where an existing beacon or sensor is located.
    fn detectable_and_empty(&self, location: (isize, isize)) -> bool {
        self.within_detection_range(location) && !self.known_location(location)
    }

    fn known_location(&self, location: (isize, isize)) -> bool {
        self.location == location || self.detected_beacon == location
    }

    fn max_x_visible(&self) -> isize {
        self.location.0 + self.beacon_distance as isize
    }

    fn max_x_visible_on_row(&self, row: isize) -> Option<isize> {
        let y_offset = abs_distance(self.location.1, row);

        // the provided row is outside of our detection range, we can't see anything
        if self.beacon_distance > y_offset {
            return None;
        }

        Some(self.beacon_distance as isize + y_offset as isize)
    }

    fn max_y_visible(&self) -> isize {
        self.location.1 + self.beacon_distance as isize
    }

    fn min_x_visible(&self) -> isize {
        self.location.0 - self.beacon_distance as isize
    }

    fn min_y_visible(&self) -> isize {
        self.location.1 - self.beacon_distance as isize
    }

    fn new(location: (isize, isize), detected_beacon: (isize, isize)) -> Self {
        let beacon_distance = manhattan_distance(location, detected_beacon);

        Self {
            location,
            detected_beacon,
            beacon_distance,
        }
    }

    fn within_detection_range(&self, other_location: (isize, isize)) -> bool {
        manhattan_distance(self.location, other_location) <= self.beacon_distance
    }
}

fn abs_distance(left: isize, right: isize) -> usize {
    (left - right).abs() as usize
}

#[cfg(test)]
fn debug_print(environment: &Environment, bounds: (isize, isize, isize, isize)) {
    let mut output = String::new();

    println!("bounds: {:?}", bounds);

    for y in std::ops::RangeInclusive::new(bounds.1 - 1, bounds.3 + 1) {
        output.push_str(&format!("{:3} ", y));

        for x in std::ops::RangeInclusive::new(bounds.0 - 1, bounds.2 + 1) {
            if environment.sensors.iter().any(|s| s.location == (x, y)) {
                output.push_str("S");
            } else if environment
                .sensors
                .iter()
                .any(|s| s.detected_beacon == (x, y))
            {
                output.push_str("B");
            } else if environment
                .sensors
                .iter()
                .any(|s| s.within_detection_range((x, y)))
            {
                output.push_str("#");
            } else {
                output.push_str(".");
            }
        }

        output.push_str("\n");
    }

    println!("{output}");
}

fn main() {
    let environment = parse_environment(INPUT_DATA);

    let detectable_positions = environment.detectable_positions_within_row(2_000_000);
    println!("detectable positions: {detectable_positions}");

    let possible_beacon_positions = environment.search_within_bounds((0, 0, 4_000_000, 4));
    println!("unknown beacon location within bounds: {:?}", possible_beacon_positions);
}

fn manhattan_distance(left: (isize, isize), right: (isize, isize)) -> usize {
    abs_distance(left.0, right.0) + abs_distance(left.1, right.1)
}

fn parse_environment(data: &[u8]) -> Environment {
    let data = std::str::from_utf8(data).unwrap();
    let sensors: Vec<Sensor> = data.lines().map(|l| parse_line(l)).collect();

    Environment::new(sensors)
}

fn parse_line(line: &str) -> Sensor {
    let captures = LINE_MATCH.captures(line).unwrap();

    let sensor_loc: (isize, isize) = (captures[1].parse().unwrap(), captures[2].parse().unwrap());
    let beacon_loc: (isize, isize) = (captures[3].parse().unwrap(), captures[4].parse().unwrap());

    Sensor::new(sensor_loc, beacon_loc)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static [u8] = include_bytes!("../data/sample");

    #[test]
    fn test_environment_with_sample() {
        let environment = parse_environment(SAMPLE_INPUT);
        debug_print(&environment, environment.aabb());

        let relevant_sensor_count = environment.sensors_within_range_of_row(10).count();
        assert_eq!(relevant_sensor_count, 6);

        let relevant_range = environment.relevant_row_range(10);
        assert_eq!(relevant_range, -8..28);

        let detectable_positions = environment.detectable_positions_within_row(10);
        assert_eq!(detectable_positions, 26);

        let bounds = (0, 0, 20, 20);
        let unknown_beacon_position = environment.search_within_bounds(bounds).unwrap();

        debug_print(&environment, bounds);
        debug_print(&environment, (unknown_beacon_position.0, unknown_beacon_position.1, unknown_beacon_position.0, unknown_beacon_position.1));

        assert_eq!(unknown_beacon_position, (14, 11));
    }

    #[test]
    fn test_line_parsing() {
        let data = std::str::from_utf8(SAMPLE_INPUT).unwrap();

        let sensors: Vec<Sensor> = data.lines().map(|l| parse_line(l)).collect();
        assert_eq!(sensors.len(), 14);

        let expected_sensors = vec![
            Sensor {
                location: (2, 18),
                detected_beacon: (-2, 15),
                beacon_distance: 7,
            },
            Sensor {
                location: (9, 16),
                detected_beacon: (10, 16),
                beacon_distance: 1,
            },
            Sensor {
                location: (13, 2),
                detected_beacon: (15, 3),
                beacon_distance: 3,
            },
            Sensor {
                location: (12, 14),
                detected_beacon: (10, 16),
                beacon_distance: 4,
            },
            Sensor {
                location: (10, 20),
                detected_beacon: (10, 16),
                beacon_distance: 4,
            },
            Sensor {
                location: (14, 17),
                detected_beacon: (10, 16),
                beacon_distance: 5,
            },
            Sensor {
                location: (8, 7),
                detected_beacon: (2, 10),
                beacon_distance: 9,
            },
            Sensor {
                location: (2, 0),
                detected_beacon: (2, 10),
                beacon_distance: 10,
            },
            Sensor {
                location: (0, 11),
                detected_beacon: (2, 10),
                beacon_distance: 3,
            },
            Sensor {
                location: (20, 14),
                detected_beacon: (25, 17),
                beacon_distance: 8,
            },
            Sensor {
                location: (17, 20),
                detected_beacon: (21, 22),
                beacon_distance: 6,
            },
            Sensor {
                location: (16, 7),
                detected_beacon: (15, 3),
                beacon_distance: 5,
            },
            Sensor {
                location: (14, 3),
                detected_beacon: (15, 3),
                beacon_distance: 1,
            },
            Sensor {
                location: (20, 1),
                detected_beacon: (15, 3),
                beacon_distance: 7,
            },
        ];

        for (actual, expected) in sensors.iter().zip(expected_sensors) {
            assert_eq!(actual, &expected);
        }
    }

    #[test]
    fn test_ranging_functions() {
        assert_eq!(abs_distance(0, 5), 5);
        assert_eq!(abs_distance(-20, -20), 0);
        assert_eq!(abs_distance(-10, 10), 20);

        assert_eq!(manhattan_distance((0, 0), (0, 0)), 0);
        assert_eq!(manhattan_distance((-10, 0), (0, 10)), 20);
        assert_eq!(manhattan_distance((0, 5), (5, 5)), 5);

        let sensor = Sensor::new((0, 0), (5, 5));

        assert!(sensor.can_detect_row(0));
        assert!(sensor.can_detect_row(10));
        assert!(sensor.can_detect_row(-10));

        assert!(!sensor.can_detect_row(11));
        assert!(!sensor.can_detect_row(-11));

        assert_eq!(sensor.min_x_visible(), -10);
        assert_eq!(sensor.max_x_visible(), 10);
    }
}
