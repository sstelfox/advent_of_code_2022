use lazy_static::lazy_static;
use regex::Regex;

const INPUT_DATA: &'static [u8] = include_bytes!("../data/input");

lazy_static! {
    static ref LINE_MATCH: Regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
}

#[derive(Debug)]
struct Environment {
    sensors: Vec<Sensor>,
}

impl Environment {
    fn new(sensors: Vec<Sensor>) -> Environment {
        Self { sensors }
    }
}

fn abs_distance(left: isize, right: isize) -> isize {
    (left - right).abs()
}

#[derive(Debug, Eq, PartialEq)]
struct Sensor {
    location: (isize, isize),

    detected_beacon: (isize, isize),
    manhattan_distance: isize,
}

impl Sensor {
    fn new(location: (isize, isize), detected_beacon: (isize, isize)) -> Self {
        let manhattan_distance = abs_distance(location.0, detected_beacon.0) + abs_distance(location.1, detected_beacon.1);

        Self {
            location,
            detected_beacon,
            manhattan_distance,
        }
    }
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

fn main() {
    let _environment = parse_environment(INPUT_DATA);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static [u8] = include_bytes!("../data/sample");

    #[test]
    fn test_environment_generation() {
        let _environment = parse_environment(SAMPLE_INPUT);
    }

    #[test]
    fn test_line_parsing() {
        let data = std::str::from_utf8(SAMPLE_INPUT).unwrap();

        let sensors: Vec<Sensor> = data.lines().map(|l| parse_line(l)).collect();
        assert_eq!(sensors.len(), 14);

        let expected_sensors = vec![
            Sensor { location: (2, 18), detected_beacon: (-2, 15), manhattan_distance: 7 },
            Sensor { location: (9, 16), detected_beacon: (10, 16), manhattan_distance: 1 },
            Sensor { location: (13, 2), detected_beacon: (15, 3), manhattan_distance: 3 },
            Sensor { location: (12, 14), detected_beacon: (10, 16), manhattan_distance: 4 },
            Sensor { location: (10, 20), detected_beacon: (10, 16), manhattan_distance: 4 },
            Sensor { location: (14, 17), detected_beacon: (10, 16), manhattan_distance: 5 },
            Sensor { location: (8, 7), detected_beacon: (2, 10), manhattan_distance: 9 },
            Sensor { location: (2, 0), detected_beacon: (2, 10), manhattan_distance: 10 },
            Sensor { location: (0, 11), detected_beacon: (2, 10), manhattan_distance: 3 },
            Sensor { location: (20, 14), detected_beacon: (25, 17), manhattan_distance: 8 },
            Sensor { location: (17, 20), detected_beacon: (21, 22), manhattan_distance: 6 },
            Sensor { location: (16, 7), detected_beacon: (15, 3), manhattan_distance: 5 },
            Sensor { location: (14, 3), detected_beacon: (15, 3), manhattan_distance: 1 },
            Sensor { location: (20, 1), detected_beacon: (15, 3), manhattan_distance: 7 },
        ];

        for (actual, expected) in sensors.iter().zip(expected_sensors) {
            assert_eq!(actual, &expected);
        }
    }
}
