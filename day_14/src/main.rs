const INPUT_DATA: &'static [u8] = include_bytes!("../data/input");

const SIMULATION_HEIGHT: usize = 1024;
const SIMULATION_WIDTH: usize = 1024;

const SPAWNER_X: isize = 500;
const SPAWNER_Y: isize = 0;

struct SimulatedEnvironment {
    aabb: (Point, Point),
    active_sand: Option<Point>,
    spawner_location: Point,

    tiles: Vec<Tile>,
}

impl SimulatedEnvironment {
    fn display_string(&self) -> String {
        let mut output: Vec<String> = vec![];

        for y in std::ops::RangeInclusive::new(self.aabb.0.y, self.aabb.1.y) {
            let mut line_chars = vec![];

            for x in std::ops::RangeInclusive::new(self.aabb.0.x, self.aabb.1.x) {
                line_chars.push(self.get_tile(x, y).char());
            }

            output.push(line_chars.into_iter().collect());
        }

        output.join("\n")
    }

    fn draw_line_segment(&mut self, line: LineSegment) {
        if line.horizontal() {
            let (min_x, max_x) = (
                line.left.x.min(line.right.x),
                line.left.x.max(line.right.x)
            );

            self.aabb.0.x = min_x.min(self.aabb.0.x);
            self.aabb.1.x = max_x.max(self.aabb.1.x);

            for x in std::ops::RangeInclusive::new(min_x, max_x) {
                self.set_tile(x, line.left.y, Tile::Rock);
            }
        } else if line.vertical() {
            let (min_y, max_y) = (
                line.left.y.min(line.right.y),
                line.left.y.max(line.right.y)
            );

            self.aabb.0.y = min_y.min(self.aabb.0.y);
            self.aabb.1.y = max_y.max(self.aabb.1.y);

            for y in std::ops::RangeInclusive::new(min_y, max_y) {
                self.set_tile(line.left.x, y, Tile::Rock);
            }
        } else {
            panic!("does not support diagonal lines: {:?}", line);
        }
    }

    fn get_tile(&self, x: isize, y: isize) -> Tile {
        self.tiles[x as usize + y as usize * SIMULATION_WIDTH]
    }

    fn new(spawner_location: Point) -> Self {
        let mut sim_env = SimulatedEnvironment {
            aabb: (spawner_location, spawner_location),
            active_sand: None,
            spawner_location,

            tiles: vec![Tile::default(); SIMULATION_WIDTH * SIMULATION_HEIGHT],
        };

        sim_env.set_tile(spawner_location.x, spawner_location.y, Tile::Spawner);

        sim_env
    }

    fn set_tile(&mut self, x: isize, y: isize, tile: Tile) {
        if self.spawner_location.x == x && self.spawner_location.y == y && tile != Tile::Spawner{
            println!("attempted overwriting of the spawner location...");
            return;
        }

        self.tiles[x as usize + y as usize * SIMULATION_WIDTH] = tile;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct LineSegment {
    left: Point,
    right: Point,
}

impl LineSegment {
    fn horizontal(&self) -> bool {
        self.left.x != self.right.x && self.left.y == self.right.y
    }

    fn quad(left_x: isize, left_y: isize, right_x: isize, right_y: isize) -> LineSegment {
        Self {
            left: Point::new(left_x, left_y),
            right: Point::new(right_x, right_y),
        }
    }

    fn vertical(&self) -> bool {
        self.left.x == self.right.x && self.left.y != self.right.y
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Point {
        let str_coords: Vec<isize> = value.split(",").map(|n| n.parse::<isize>().unwrap()).take(2).collect();
        Point { x: str_coords[0], y: str_coords[1] }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Rock,
    Sand(bool),
    Spawner,
}

impl Tile {
    fn char(&self) -> char {
        use Tile::*;

        match self {
            Empty => '.',
            Rock => '#',
            Sand(active) => { if *active { 'A' } else { 'o' } },
            Spawner => '+',
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
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
    let spawner_location = Point::new(SPAWNER_X, SPAWNER_Y);
    let mut simulated_environment = SimulatedEnvironment::new(spawner_location);

    let data = std::str::from_utf8(data).unwrap();
    let lines_to_draw: Vec<LineSegment> = data.lines().map(|l| parse_line(l)).flatten().collect();

    lines_to_draw.into_iter()
        .for_each(|l| simulated_environment.draw_line_segment(l));

    simulated_environment
}

fn main() {
    let sim_env = parse_simulated_environment(INPUT_DATA);

    println!("AABB: (({}, {}), ({}, {}))", sim_env.aabb.0.x, sim_env.aabb.0.y, sim_env.aabb.1.x, sim_env.aabb.1.y);
    println!("{}", sim_env.display_string());
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

        assert_eq!(line_segments, expected_line_segments);
    }

    #[test]
    fn test_simulation_parsing() {
        let sim_env = parse_simulated_environment(SAMPLE_INPUT);
    }
}
