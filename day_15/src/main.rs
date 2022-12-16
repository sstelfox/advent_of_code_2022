use lazy_static::lazy_static;
use regex::Regex;

const INPUT_DATA: &'static [u8] = include_bytes!("../data/input");

lazy_static! {
    static ref LINE_MATCH: Regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
}

#[derive(Debug)]
struct Environment {
    size: (isize, isize),
    tiles: Vec<Tile>,
}

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
enum Tile {
    Unknown,
    Beacon,
    NoBeacon,
    Sensor,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Unknown
    }
}

fn parse_line(line: &str) -> (Point, Point) {
    let captures = LINE_MATCH.captures(line).unwrap();

    let sensor = Point::new(captures[1].parse().unwrap(), captures[2].parse().unwrap());
    let beacon = Point::new(captures[3].parse().unwrap(), captures[4].parse().unwrap());

    (sensor, beacon)
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static [u8] = include_bytes!("../data/sample");

    #[test]
    fn test_line_parsing() {
        let data = std::str::from_utf8(SAMPLE_INPUT).unwrap();

        let locations: Vec<(Point, Point)> = data.lines().map(|l| parse_line(l)).collect();
        assert_eq!(locations.len(), 14);

        let expected_locations = vec![
            (Point::new(2, 18), Point::new(-2, 15)),
            (Point::new(9, 16), Point::new(10, 16)),
            (Point::new(13, 2), Point::new(15, 3)),
            (Point::new(12, 14), Point::new(10, 16)),
            (Point::new(10, 20), Point::new(10, 16)),
            (Point::new(14, 17), Point::new(10, 16)),
            (Point::new(8, 7), Point::new(2, 10)),
            (Point::new(2, 0), Point::new(2, 10)),
            (Point::new(0, 11), Point::new(2, 10)),
            (Point::new(20, 14), Point::new(25, 17)),
            (Point::new(17, 20), Point::new(21, 22)),
            (Point::new(16, 7), Point::new(15, 3)),
            (Point::new(14, 3), Point::new(15, 3)),
            (Point::new(20, 1), Point::new(15, 3)),
        ];

        assert_eq!(locations, expected_locations);
    }
}
