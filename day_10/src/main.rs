const INPUT_DATA: &'static [u8] = include_bytes!("../data/input");

const DISPLAY_HEIGHT: usize = 6;
const DISPLAY_WIDTH: usize = 40;

const PIXEL_COUNT: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

struct Cpu {
    instructions: Vec<Operation>,

    instruction_counter: usize,
    cycle_counter: usize,

    register_x: isize, // also the sprite position

    display: [bool; PIXEL_COUNT],
    signal_strength: Option<isize>,

    pending_cycles: Option<usize>,
}

impl Cpu {
    fn current_operation(&self) -> Option<Operation> {
        self.instructions.get(self.instruction_counter).copied()
    }

    fn current_pixel_index(&self) -> usize {
        // The cycle counter advances before we update the display, but the display is zero-indexed
        // so we need to reduce it by one
        (self.cycle_counter - 1) % PIXEL_COUNT
    }

    fn display_string(&self) -> String {
        let row_strs: Vec<String> = self.display
            .chunks(DISPLAY_WIDTH)
            .map(|row| {
                row.iter().map(|px| { if *px { "#" } else { "." } }).collect::<String>()
            })
            .collect();

        row_strs.join("\n")
    }

    fn in_sprite_window(&self) -> bool {
        let (min, max) = self.sprite_window();

        // We only look at the horizontal position to determine if we're in the
        // sprite window
        let pixel_loc = self.current_pixel_index() % DISPLAY_WIDTH;

        min <= pixel_loc && pixel_loc <= max
    }

    fn new(instructions: Vec<Operation>) -> Self {
        Cpu {
            instructions,

            instruction_counter: 0,
            cycle_counter: 0,

            register_x: 1,

            display: [false; PIXEL_COUNT],
            signal_strength: None,

            pending_cycles: None,
        }
    }

    fn run_with_signal_strengths(&mut self) -> Vec<isize> {
        let mut signal_strengths = vec![];

        while self.tick() {
            if ((self.cycle_counter + 20) % 40) == 0 {
                signal_strengths.push(self.signal_strength.unwrap());
            }
        }

        signal_strengths
    }

    fn sprite_window(&self) -> (usize, usize) {
        let min = (self.register_x - 1).max(0) as usize;
        let max = (self.register_x + 1).min(DISPLAY_WIDTH as isize - 1) as usize;

        (min, max)
    }

    fn tick(&mut self) -> bool {
        if let Some(op) = self.current_operation() {
            self.cycle_counter += 1;

            if let Some(rem) = self.pending_cycles {
                self.pending_cycles = Some(rem - 1);
            } else {
                self.pending_cycles = Some(op.cycle_count() - 1);
            }

            self.update_display();
            self.update_signal_strength();

            if self.pending_cycles == Some(0) {
                self.pending_cycles = None;
                op.apply(self);
                self.instruction_counter += 1;
            }

            true
        } else {
            false
        }
    }

    fn update_display(&mut self) {
        if self.in_sprite_window() {
            self.display[self.current_pixel_index()] = true;
        }
    }

    fn update_signal_strength(&mut self) {
        self.signal_strength = Some(self.register_x * self.cycle_counter as isize);
    }
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field("instruction_counter", &self.instruction_counter)
            .field("cycle_counter", &self.cycle_counter)
            .field("register_x", &self.register_x)
            .field("current_operation", &self.current_operation())
            .field("pending_cycles", &self.pending_cycles)
            .field("signal_strength", &self.signal_strength)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operation {
    Noop,
    AddX(isize),
}

impl Operation {
    fn apply(&self, cpu: &mut Cpu) {
        use Operation::*;

        match self {
            AddX(val) => {
                cpu.register_x += val;
            }
            _ => (),
        }
    }

    fn cycle_count(&self) -> usize {
        use Operation::*;

        match self {
            Noop => 1,
            AddX(_) => 2,
        }
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Operation {
        use Operation::*;

        let instruction: Vec<&str> = value.split_whitespace().take(2).collect();
        match (instruction.get(0), instruction.get(1)) {
            (Some(&"noop"), None) => Noop,
            (Some(&"addx"), Some(val)) => AddX(val.parse().unwrap()),
            _ => {
                panic!("{:?}", instruction);
            }
        }
    }
}

fn parse_program(data: &[u8]) -> Vec<Operation> {
    let data = std::str::from_utf8(data).unwrap();
    data.lines().map(|l| Operation::from(l)).collect()
}

fn main() {
    let input_program = parse_program(INPUT_DATA);
    let mut cpu = Cpu::new(input_program);

    let signal_strengths = cpu.run_with_signal_strengths();
    let signal_sum: isize = signal_strengths.iter().sum();

    println!("{:?}", signal_sum);
    println!("Display:\n{}", cpu.display_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const MIN_SAMPLE: &'static [u8] = "noop\naddx 3\naddx -5".as_bytes();

    const SAMPLE_INPUT: &'static [u8] = include_bytes!("../data/sample");

    #[test]
    fn test_cpu_running() {
        use Operation::*;

        let mut cpu = Cpu::new(vec![Noop, AddX(3), AddX(-5)]);

        // Check the initial state
        assert_eq!(cpu.current_operation(), Some(Noop));
        assert_eq!(cpu.cycle_counter, 0);
        assert_eq!(cpu.register_x, 1);

        // Check the state after the first cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), Some(AddX(3)));
        assert_eq!(cpu.cycle_counter, 1);
        assert_eq!(cpu.register_x, 1);

        // Check the state after the second cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), Some(AddX(3)));
        assert_eq!(cpu.cycle_counter, 2);
        assert_eq!(cpu.register_x, 1);

        // Third cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), Some(AddX(-5)));
        assert_eq!(cpu.cycle_counter, 3);
        assert_eq!(cpu.register_x, 4);

        // Fourth cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), Some(AddX(-5)));
        assert_eq!(cpu.cycle_counter, 4);
        assert_eq!(cpu.register_x, 4);

        // Fifth cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), None);
        assert_eq!(cpu.cycle_counter, 5);
        assert_eq!(cpu.register_x, -1);

        // All future ticks
        assert!(!cpu.tick());
        assert_eq!(cpu.cycle_counter, 5);
    }

    #[test]
    fn test_full_sample_parsing() {
        use Operation::*;

        let program = parse_program(MIN_SAMPLE);
        assert_eq!(vec![Noop, AddX(3), AddX(-5)], program);
    }

    #[test]
    fn test_min_sample_program() {
        let program = parse_program(SAMPLE_INPUT);
        let mut cpu = Cpu::new(program);
        let signal_strengths = cpu.run_with_signal_strengths();

        assert_eq!(6, signal_strengths.len());
        assert_eq!(vec![420, 1140, 1800, 2940, 2880, 3960], signal_strengths);

        let signal_sum: isize = signal_strengths.iter().sum();
        assert_eq!(13140, signal_sum);
    }

    #[test]
    fn test_sprite_window() {
        let mut cpu = Cpu::new(vec![]);

        assert_eq!(1, cpu.register_x);
        assert_eq!((0, 2), cpu.sprite_window());

        cpu.register_x = 0;
        assert_eq!((0, 1), cpu.sprite_window());

        cpu.register_x = (DISPLAY_WIDTH - 1) as isize;
        assert_eq!((38, 39), cpu.sprite_window());
    }

    #[test]
    fn test_current_pixel_index() {
        let mut cpu = Cpu::new(vec![]);

        cpu.cycle_counter = 56;
        assert_eq!(55, cpu.current_pixel_index());

        cpu.cycle_counter = PIXEL_COUNT;
        assert_eq!(PIXEL_COUNT - 1, cpu.current_pixel_index());

        cpu.cycle_counter = PIXEL_COUNT + 43;
        assert_eq!(42, cpu.current_pixel_index());
    }

    #[test]
    fn test_in_sprite_window() {
        let mut cpu = Cpu::new(vec![]);

        cpu.cycle_counter = 24;

        cpu.register_x = 21;
        assert!(!cpu.in_sprite_window());

        cpu.register_x = 22;
        assert!(cpu.in_sprite_window());

        cpu.register_x = 23;
        assert!(cpu.in_sprite_window());

        cpu.register_x = 24;
        assert!(cpu.in_sprite_window());

        cpu.register_x = 25;
        assert!(!cpu.in_sprite_window());
    }

    #[test]
    fn test_display_rendering() {
        let mut cpu = Cpu::new(vec![]);

        cpu.display[0] = true;
        let expected_display = "#.......................................\n\
                                ........................................\n\
                                ........................................\n\
                                ........................................\n\
                                ........................................\n\
                                ........................................";
        assert_eq!(expected_display, cpu.display_string());

        cpu.display[DISPLAY_WIDTH - 1] = true;
        let expected_display = "#......................................#\n\
                                ........................................\n\
                                ........................................\n\
                                ........................................\n\
                                ........................................\n\
                                ........................................";
        assert_eq!(expected_display, cpu.display_string());

        cpu.display[PIXEL_COUNT - DISPLAY_WIDTH] = true;
        let expected_display = "#......................................#\n\
                                ........................................\n\
                                ........................................\n\
                                ........................................\n\
                                ........................................\n\
                                #.......................................";
        assert_eq!(expected_display, cpu.display_string());

        cpu.display[PIXEL_COUNT - 1] = true;
        let expected_display = "#......................................#\n\
                                ........................................\n\
                                ........................................\n\
                                ........................................\n\
                                ........................................\n\
                                #......................................#";
        assert_eq!(expected_display, cpu.display_string());
    }

    #[test]
    fn test_sample_display_output() {
        let expected_display = "##..##..##..##..##..##..##..##..##..##..\n\
                                ###...###...###...###...###...###...###.\n\
                                ####....####....####....####....####....\n\
                                #####.....#####.....#####.....#####.....\n\
                                ######......######......######......####\n\
                                #######.......#######.......#######.....";

        let program = parse_program(SAMPLE_INPUT);

        let mut cpu = Cpu::new(program);
        cpu.run_with_signal_strengths();

        assert_eq!(cpu.cycle_counter, 240);
        assert_eq!(expected_display, cpu.display_string());
    }
}
