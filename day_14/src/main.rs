const INPUT_DATA: &'static [u8] = include_bytes!("../data/input");

const SIMULATION_HEIGHT: usize = 1024;
const SIMULATION_WIDTH: usize = 1024;

const SPAWNER_X: isize = 500;
const SPAWNER_Y: isize = 0;

const SEARCH_OFFSETS: [(isize, isize); 3] = [
    (0, 1), (-1, 1), (1, 1),
];

struct SimulatedEnvironment {
    aabb: (Point, Point),
    active_sand: Option<Point>,
    spawner_location: Point,

    has_floor: bool,
    path_tracing: bool,

    tiles: Vec<Tile>,
}

impl SimulatedEnvironment {
    fn add_floor(&mut self) {
        self.has_floor = true;

        let floor_height = self.aabb.1.y + 2;
        self.aabb.1.y = floor_height;

        for x in (std::ops::Range { start: 0, end: SIMULATION_WIDTH }) {
            self.set_tile(x as isize, floor_height, Tile::Rock);
        }
    }

    fn count_resting_sand(&self) -> usize {
        self.tiles.iter().filter(|t| Tile::Sand(false) == **t).count()
    }

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
        if !(line.horizontal() || line.vertical()) {
            panic!("does not support diagonal lines: {:?}", line);
        }

        let (min_x, max_x) = (
            line.left.x.min(line.right.x),
            line.left.x.max(line.right.x)
        );

        let (min_y, max_y) = (
            line.left.y.min(line.right.y),
            line.left.y.max(line.right.y)
        );

        self.update_aabb(min_x, min_y);
        self.update_aabb(max_x, max_y);

        for y in std::ops::RangeInclusive::new(min_y, max_y) {
            for x in std::ops::RangeInclusive::new(min_x, max_x) {
                self.set_tile(x, y, Tile::Rock);
            }
        }
    }

    fn enable_path_tracing(&mut self) {
        self.path_tracing = true;
    }

    fn get_tile(&self, x: isize, y: isize) -> Tile {
        self.tiles[x as usize + y as usize * SIMULATION_WIDTH]
    }

    fn new(spawner_location: Point) -> Self {
        let mut sim_env = SimulatedEnvironment {
            aabb: (spawner_location, spawner_location),
            active_sand: None,
            spawner_location,

            has_floor: false,
            path_tracing: false,

            tiles: vec![Tile::default(); SIMULATION_WIDTH * SIMULATION_HEIGHT],
        };

        sim_env.set_tile(spawner_location.x, spawner_location.y, Tile::Spawner);

        sim_env
    }

    fn set_tile(&mut self, x: isize, y: isize, tile: Tile) {
        if self.spawner_location.x == x && self.spawner_location.y == y && tile != Tile::Spawner {
            println!("attempted overwriting of the spawner location...");
            return;
        }

        self.tiles[x as usize + y as usize * SIMULATION_WIDTH] = tile;
    }

    fn tick(&mut self) -> Option<bool> {
        if let Some(sand) = self.active_sand {
            let next_loc = SEARCH_OFFSETS.iter()
                .map(|(ox, oy)| (sand.x + ox, sand.y + oy))
                .find(|(x, y)| { self.get_tile(*x, *y).is_empty() });

            if let Some((new_x, new_y)) = next_loc {
                if self.has_floor {
                    self.update_aabb(new_x, new_y);
                }

                let new_blank_tile = if self.path_tracing {
                    Tile::Path
                } else {
                    Tile::Empty
                };

                self.set_tile(sand.x, sand.y, new_blank_tile);

                if !self.within_bounds(new_x, new_y) {
                    // Sand left our active map, that's our completion status, mark it the last
                    // valid place we were at, clean up a bit and exit
                    self.active_sand = None;
                    return None;
                }

                self.set_tile(new_x, new_y, Tile::Sand(true));
                self.active_sand = Some(Point::new(new_x, new_y));

                Some(true)
            } else {
                // Sand didn't move, it'll stay here
                self.active_sand = None;
                self.set_tile(sand.x, sand.y, Tile::Sand(false));

                Some(false)
            }
        } else {
            let next_loc = SEARCH_OFFSETS.iter()
                .map(|(ox, oy)| (self.spawner_location.x + ox, self.spawner_location.y + oy))
                .find(|(x, y)| { self.get_tile(*x, *y).is_empty() });

            if let Some((sx, sy)) = next_loc {
                self.active_sand = Some(Point::new(sx, sy));
                self.set_tile(sx, sy, Tile::Sand(true));

                Some(true)
            } else {
                // We can't spawn a new moving sand tile at the target location exit early
                println!("stopped since we're unable to spawn new sand");

                None
            }
        }
    }

    fn tick_one_sand(&mut self) -> bool {
        loop {
            match self.tick() {
                Some(true) => (),                   // sand moved, keep ticking
                Some(false) => { return true; },    // sand didn't move but found a resting place, this method is done
                None => { return false; },          // sand went out of bounds or was unable to spawn, the sim is done
            }
        }
    }

    fn tick_till_done(&mut self) {
        while self.tick_one_sand() {}
    }

    fn update_aabb(&mut self, x: isize, y: isize) {
        self.aabb.0.x = x.min(self.aabb.0.x);
        self.aabb.1.x = x.max(self.aabb.1.x);

        self.aabb.0.y = y.min(self.aabb.0.y);
        self.aabb.1.y = y.max(self.aabb.1.y);
    }

    fn within_bounds(&self, x: isize, y: isize) -> bool {
        x >= self.aabb.0.x && x <= self.aabb.1.x &&
            y >= self.aabb.0.y && y <= self.aabb.1.y
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
    Path,
    Rock,
    Sand(bool),
    Spawner,
}

impl Tile {
    fn char(&self) -> char {
        use Tile::*;

        match self {
            Empty => '.',
            Path => '~',
            Rock => '#',
            Sand(active) => { if *active { 'A' } else { 'o' } },
            Spawner => '+',
        }
    }

    fn is_empty(&self) -> bool {
        use Tile::*;

        match self {
            Empty => true,
            Path => true,
            _ => false,
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
    let mut sim_env = parse_simulated_environment(INPUT_DATA);

    sim_env.enable_path_tracing();
    sim_env.tick_till_done();

    println!("final count day 1: {}", sim_env.count_resting_sand());

    let mut sim_env = parse_simulated_environment(INPUT_DATA);

    sim_env.add_floor();
    sim_env.enable_path_tracing();
    sim_env.tick_till_done();

    println!("{}", sim_env.display_string());
    println!("final count day 2: {}", sim_env.count_resting_sand());
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

        let expected_display = "......+...\n\
                                ..........\n\
                                ..........\n\
                                ..........\n\
                                ....#...##\n\
                                ....#...#.\n\
                                ..###...#.\n\
                                ........#.\n\
                                ........#.\n\
                                #########.";

        assert_eq!(expected_display, sim_env.display_string());
    }

    #[test]
    fn test_simulation_ticks() {
        let mut sim_env = parse_simulated_environment(SAMPLE_INPUT);

        assert_eq!(sim_env.tick(), Some(true));
        let expected_display = "......+...\n\
                                ......A...\n\
                                ..........\n\
                                ..........\n\
                                ....#...##\n\
                                ....#...#.\n\
                                ..###...#.\n\
                                ........#.\n\
                                ........#.\n\
                                #########.";
        assert_eq!(expected_display, sim_env.display_string());

        assert_eq!(sim_env.tick(), Some(true));
        let expected_display = "......+...\n\
                                ..........\n\
                                ......A...\n\
                                ..........\n\
                                ....#...##\n\
                                ....#...#.\n\
                                ..###...#.\n\
                                ........#.\n\
                                ........#.\n\
                                #########.";
        assert_eq!(expected_display, sim_env.display_string());

        assert!(sim_env.tick_one_sand());
        let expected_display = "......+...\n\
                                ..........\n\
                                ..........\n\
                                ..........\n\
                                ....#...##\n\
                                ....#...#.\n\
                                ..###...#.\n\
                                ........#.\n\
                                ......o.#.\n\
                                #########.";
        assert_eq!(expected_display, sim_env.display_string());

        assert!(sim_env.tick_one_sand());
        let expected_display = "......+...\n\
                                ..........\n\
                                ..........\n\
                                ..........\n\
                                ....#...##\n\
                                ....#...#.\n\
                                ..###...#.\n\
                                ........#.\n\
                                .....oo.#.\n\
                                #########.";
        assert_eq!(expected_display, sim_env.display_string());

        for _ in 3..=5 {
            assert!(sim_env.tick_one_sand());
        }

        let expected_display = "......+...\n\
                                ..........\n\
                                ..........\n\
                                ..........\n\
                                ....#...##\n\
                                ....#...#.\n\
                                ..###...#.\n\
                                ......o.#.\n\
                                ....oooo#.\n\
                                #########.";
        assert_eq!(expected_display, sim_env.display_string());

        for _ in 6..=22 {
            assert!(sim_env.tick_one_sand());
        }

        let expected_display = "......+...\n\
                                ..........\n\
                                ......o...\n\
                                .....ooo..\n\
                                ....#ooo##\n\
                                ....#ooo#.\n\
                                ..###ooo#.\n\
                                ....oooo#.\n\
                                ...ooooo#.\n\
                                #########.";
        assert_eq!(expected_display, sim_env.display_string());

        for _ in 23..=24 {
            assert!(sim_env.tick_one_sand());
        }

        let expected_display = "......+...\n\
                                ..........\n\
                                ......o...\n\
                                .....ooo..\n\
                                ....#ooo##\n\
                                ...o#ooo#.\n\
                                ..###ooo#.\n\
                                ....oooo#.\n\
                                .o.ooooo#.\n\
                                #########.";
        assert_eq!(expected_display, sim_env.display_string());

        assert!(!sim_env.tick_one_sand());

        let expected_display = "......+...\n\
                                ..........\n\
                                ......o...\n\
                                .....ooo..\n\
                                ....#ooo##\n\
                                ...o#ooo#.\n\
                                ..###ooo#.\n\
                                ....oooo#.\n\
                                .o.ooooo#.\n\
                                #########.";
        assert_eq!(expected_display, sim_env.display_string());
    }

    #[test]
    fn test_simulation_count() {
        let mut sim_env = parse_simulated_environment(SAMPLE_INPUT);
        sim_env.tick_till_done();
        assert_eq!(sim_env.count_resting_sand(), 24);
    }

    #[test]
    fn test_day_2_floor_simulation() {
        let mut sim_env = parse_simulated_environment(SAMPLE_INPUT);

        sim_env.add_floor();
        sim_env.tick_till_done();

        assert_eq!(sim_env.count_resting_sand(), 93);
    }
}
