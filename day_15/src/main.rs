use lazy_static::lazy_static;
use regex::Regex;

const INPUT_DATA: &'static [u8] = include_bytes!("../data/input");

lazy_static! {
    static ref LINE_MATCH: Regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
}

#[derive(Debug)]
struct Environment {
    position_offsets: (isize, isize),
    size: (usize, usize),

    tiles: Vec<Tile>,
}

impl Environment {
    fn add_location(&mut self, sensor: &Point, beacon: &Point) {
        let sensor = Point::new(sensor.x - self.position_offsets.0, sensor.y - self.position_offsets.1);
        self.set_tile(sensor.x as usize, sensor.y as usize, Tile::Sensor);

        let beacon = Point::new(beacon.x - self.position_offsets.0, beacon.y - self.position_offsets.1);
        self.set_tile(beacon.x as usize, beacon.y as usize, Tile::Beacon);

        let _manhattan_distance = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();

        self.tiles[0] = Tile::Unknown;
    }

    fn display_string(&self) -> String {
        let mut output: Vec<String> = vec![];

        for y in std::ops::RangeInclusive::new(0, self.size.1) {
            let mut line_chars = vec![];

            for x in std::ops::RangeInclusive::new(0, self.size.0) {
                line_chars.push(self.get_tile(x, y).char());
            }

            output.push(line_chars.into_iter().collect());
        }

        output.join("\n")
    }

    fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[x + y * self.size.1]
    }

    fn new(x_size: usize, y_size: usize, x_offset: isize, y_offset: isize) -> Environment {
        Self {
            position_offsets: (x_offset, y_offset),
            size: (x_size, y_size),

            tiles: vec![Tile::default(); x_size * y_size],
        }
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles[x + y * self.size.1] = tile;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Unknown,
    Beacon,
    NoBeacon,
    Sensor,
}

impl Tile {
    fn char(&self) -> char {
        use Tile::*;

        match self {
            Unknown => '.',
            Beacon => 'B',
            NoBeacon => '#',
            Sensor => 'S',
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Unknown
    }
}

fn parse_environment(data: &[u8]) -> Environment {
    let data = std::str::from_utf8(data).unwrap();
    let locations: Vec<(Point, Point)> = data.lines().map(|l| parse_line(l)).collect();

    let mut bounds: Option<(Point, Point)> = None;

    for location in locations.iter().map(|loc| [&loc.0, &loc.1]).flatten() {
        if let Some((ref mut min, ref mut max)) = bounds {
            min.x = min.x.min(location.x);
            max.x = max.x.max(location.x);

            min.y = min.y.min(location.y);
            max.y = max.y.max(location.y);
        } else {
            bounds = Some((location.clone(), location.clone()));
        }
    }

    let bounds = bounds.unwrap();

    let size_x = (bounds.1.x - bounds.0.x) as usize;
    let size_y = (bounds.1.y - bounds.0.y) as usize;

    let offset_x = bounds.0.x;
    let offset_y = bounds.0.y;

    let mut env = Environment::new(size_x, size_y, offset_x, offset_y);

    for (sensor, beacon) in locations.iter() {
        env.add_location(sensor, beacon);
    }

    env
}

fn parse_line(line: &str) -> (Point, Point) {
    let captures = LINE_MATCH.captures(line).unwrap();

    let sensor = Point::new(captures[1].parse().unwrap(), captures[2].parse().unwrap());
    let beacon = Point::new(captures[3].parse().unwrap(), captures[4].parse().unwrap());

    (sensor, beacon)
}

fn main() {
    let environment = parse_environment(INPUT_DATA);

    println!("{}", environment.display_string());
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
