
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

    fn is_halted(&self) -> bool {
        self.current_operation().is_none()
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operation {
    Noop,
    AddX(isize),
}

impl Operation {
    fn apply(&self, cpu: &mut Cpu) {
        use Operation::*;

        match self {
            AddX(val) => { cpu.register_x += val; },
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

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_running() {
        use Operation::*;

        let mut cpu = Cpu::new(vec![Noop, AddX(3), AddX(-5)]);

        // Check the initial state
        assert_eq!(cpu.current_operation(), Some(Noop));
        assert_eq!(cpu.program_counter, 0);
        assert_eq!(cpu.register_x, 1);
        assert!(!cpu.is_halted());

        // Check the state after the first cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), Some(AddX(3)));
        assert_eq!(cpu.program_counter, 1);
        assert_eq!(cpu.register_x, 1);
        assert!(!cpu.is_halted());

        // Check the state after the second cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), Some(AddX(3)));
        assert_eq!(cpu.program_counter, 2);
        assert_eq!(cpu.register_x, 1);
        assert!(!cpu.is_halted());

        // Third cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), Some(AddX(-5)));
        assert_eq!(cpu.program_counter, 3);
        assert_eq!(cpu.register_x, 4);
        assert!(!cpu.is_halted());

        // Fourth cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), Some(AddX(-5)));
        assert_eq!(cpu.program_counter, 4);
        assert_eq!(cpu.register_x, 4);
        assert!(!cpu.is_halted());

        // Fifth cycle
        assert!(cpu.tick());
        assert_eq!(cpu.current_operation(), None);
        assert_eq!(cpu.program_counter, 5);
        assert_eq!(cpu.register_x, -1);
        assert!(cpu.is_halted());

        // All future ticks
        assert!(!cpu.tick());
        assert_eq!(cpu.program_counter, 5);
        assert!(cpu.is_halted());
    }
}
