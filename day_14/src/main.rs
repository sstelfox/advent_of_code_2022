const INPUT_DATA: &'static [u8] = include_bytes!("../data/input");

struct SimulatedEnvironment {
}

#[derive(Debug, Eq, PartialEq)]
struct LineSegment {
    left: Point,
    right: Point,
}

impl LineSegment {
    fn quad(left_x: usize, left_y: usize, right_x: usize, right_y: usize) -> LineSegment {
        Self {
            left: Point { x: left_x, y: left_y },
            right: Point { x: right_x, y: right_y },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl From<&str> for Point {
    fn from(value: &str) -> Point {
        let str_coords: Vec<usize> = value.split(",").map(|n| n.parse::<usize>().unwrap()).take(2).collect();
        Point { x: str_coords[0], y: str_coords[1] }
    }
}

#[derive(Debug)]
enum Tile {
    Empty,
    RestingSand,
    Rock,
    SandSpawner,
}

fn parse_line(data: &str) -> Vec<LineSegment> {
    let points: Vec<Point> = data.split(" -> ")
        .map(|p| Point::from(p))
        .collect();

    points.windows(2)
        .map(|pts| LineSegment { left: pts[0], right: pts[1] })
        .collect()
}

fn parse_simulated_environment(data: &[u8]) -> SimulatedEnvironment {
    let data = std::str::from_utf8(data).unwrap();
    let lines_to_draw: Vec<LineSegment> = data.lines().map(|l| parse_line(l)).flatten().collect();

    unimplemented!()
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

        let line_segments: Vec<LineSegment> = data.lines().map(|l| parse_line(l)).flatten().collect();
        let expected_line_segments = vec![
            // first line
            LineSegment::quad(498, 4, 498, 6),
            LineSegment::quad(498, 6, 496, 6),

            // second line
            LineSegment::quad(503, 4, 502, 4),
            LineSegment::quad(502, 4, 502, 9),
            LineSegment::quad(502, 9, 494, 9),
        ];

        assert_eq!(expected_line_segments, line_segments);
    }
}
