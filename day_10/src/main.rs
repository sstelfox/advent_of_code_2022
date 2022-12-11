const INPUT_DATA: &'static [u8] = include_bytes!("../data/input");

struct Cpu {
    instructions: Vec<Operation>,

    instruction_counter: usize,
    program_counter: usize,
    register_x: isize,

    pending_cycles: Option<usize>,
}

impl Cpu {
    fn current_operation(&self) -> Option<Operation> {
        self.instructions.get(self.instruction_counter).copied()
    }

    fn new(instructions: Vec<Operation>) -> Self {
        Cpu {
            instructions,

            instruction_counter: 0,
            program_counter: 0,
            register_x: 1,

            pending_cycles: None,
        }
    }

    fn signal_strength(&self) -> isize {
        self.register_x * self.program_counter as isize
    }

    fn run_with_signal_strengths(&mut self) -> Vec<isize> {
        let mut signal_strengths = vec![];

        while self.tick() {
            if ((self.program_counter + 20) % 40) == 0 {
                println!("{:?}", self);
                signal_strengths.push(self.signal_strength());
            }
        }

        println!("final: {:?}", self);

        signal_strengths
    }

    fn tick(&mut self) -> bool {
        if let Some(op) = self.current_operation() {
            self.program_counter += 1;

            if let Some(rem) = self.pending_cycles {
                self.pending_cycles = Some(rem - 1);
            } else {
                self.pending_cycles = Some(op.cycle_count() - 1);
            }

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
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field("instruction_counter", &self.instruction_counter)
            .field("program_counter", &self.program_counter)
            .field("register_x", &self.register_x)
            .field("current_operation", &self.current_operation())
            .field("pending_cycles", &self.pending_cycles)
            .field("signal_strength", &self.signal_strength())
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
        assert_eq!(cpu.program_counter, 0);
        assert_eq!(cpu.register_x, 1);

        // Check the state after the first cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), Some(AddX(3)));
        assert_eq!(cpu.program_counter, 1);
        assert_eq!(cpu.register_x, 1);

        // Check the state after the second cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), Some(AddX(3)));
        assert_eq!(cpu.program_counter, 2);
        assert_eq!(cpu.register_x, 1);

        // Third cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), Some(AddX(-5)));
        assert_eq!(cpu.program_counter, 3);
        assert_eq!(cpu.register_x, 4);

        // Fourth cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), Some(AddX(-5)));
        assert_eq!(cpu.program_counter, 4);
        assert_eq!(cpu.register_x, 4);

        // Fifth cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), None);
        assert_eq!(cpu.program_counter, 5);
        assert_eq!(cpu.register_x, -1);

        // All future ticks
        assert!(!cpu.tick());
        assert_eq!(cpu.program_counter, 5);
    }

    #[test]
    fn test_full_sample_parsing() {
        use Operation::*;

        let program = parse_program(MIN_SAMPLE);
        assert_eq!(vec![Noop, AddX(3), AddX(-5)], program);
    }

    #[test]
    fn test_min_sample_program() {
        use Operation::*;

        let program = parse_program(SAMPLE_INPUT);
        let mut cpu = Cpu::new(program);
        let signal_strengths = cpu.run_with_signal_strengths();

        assert_eq!(6, signal_strengths.len());
        assert_eq!(vec![420, 1140, 1800, 2940, 2880, 3960], signal_strengths);

        let signal_sum: isize = signal_strengths.iter().sum();
        assert_eq!(13140, signal_sum);
    }
}
