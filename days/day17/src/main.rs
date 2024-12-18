use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

#[derive(Debug, Clone)]
struct Solver {
    a: u64,
    b: u64,
    c: u64,
    instruction_pointer: usize,
    instructions: Vec<(u8, u8)>,
}

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).collect::<Output>()
    }

    fn part_two(input: &[u8], debug: u8) -> impl Display + 'static {
        let mut solver = Self::new(input);

        let mut a = 0u64;
        let target: Vec<_> = solver
            .instructions
            .iter()
            .flat_map(|&(a, b)| [a, b])
            .collect();
        let mut digits = 0;
        let mut start_at = 0;
        while digits < target.len() {
            let i = target.len() - digits - 1;
            let b = target[i];
            if let Some(add) = the_inverse_function(a, b, start_at, &mut solver) {
                a <<= 3;
                a += add;
                digits += 1;
                start_at = 0;
            } else {
                start_at = (a % 8) + 1;
                a >>= 3;
                digits -= 1;
            }
        }

        if debug == 1 {
            solver.reset(a);
            assert!(solver.eq(target.iter().copied()));
        }
        if debug == 2 {
            let mut ma = a;
            while ma > 0 {
                let rem = ma % 8;
                eprint!("{rem},");
                ma >>= 3;
            }
            eprintln!();
        }
        a
    }
}

fn the_inverse_function(a: u64, b: u8, start_at: u64, solver: &mut Solver) -> Option<u64> {
    for possible in start_at..8 {
        let a = (a << 3) + possible;
        solver.reset(a);
        if solver.next_output() == Some(b) {
            return Some(possible);
        }
    }
    None
}

impl Solver {
    fn new(input: &[u8]) -> Self {
        let mut con = Consume::new(input);
        con.non_digits();
        let a = con.int().unwrap();
        con.non_digits();
        let b = con.int().unwrap();
        con.non_digits();
        let c = con.int().unwrap();
        con.non_digits();

        let instructions = std::iter::from_fn(|| {
            let n = con.consume_byte()? - b'0';
            con.consume_byte();
            let m = con.consume_byte()? - b'0';
            con.consume_byte();
            Some((n, m))
        })
        .collect();

        Self {
            a,
            b,
            c,
            instruction_pointer: 0,
            instructions,
        }
    }

    fn next_output(&mut self) -> Option<u8> {
        while let Some(&(opcode, operand)) = self.instructions.get(self.instruction_pointer) {
            if let Some(output) = self.step(opcode, operand) {
                return Some(output);
            }
        }
        None
    }

    /// Returns `true` when outputting
    fn step(&mut self, opcode: u8, operand: u8) -> Option<u8> {
        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => {
                return Some(self.out(operand));
            }
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("more than 3 bits"),
        }
        None
    }

    fn reset(&mut self, a: u64) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.instruction_pointer = 0;
    }

    fn combo(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("combo operand 7 is reserved"),
            _ => panic!("more than 3 bits"),
        }
    }

    fn adv(&mut self, operand: u8) {
        self.a /= 2u64.pow(self.combo(operand) as u32);
        self.instruction_pointer += 1;
    }

    fn bxl(&mut self, operand: u8) {
        self.b ^= operand as u64;
        self.instruction_pointer += 1;
    }

    fn bst(&mut self, operand: u8) {
        self.b = self.combo(operand) % 8;
        self.instruction_pointer += 1;
    }

    fn jnz(&mut self, operand: u8) {
        if self.a != 0 {
            self.instruction_pointer = operand as usize / 2;
        } else {
            self.instruction_pointer += 1;
        }
    }

    fn bxc(&mut self, _operand: u8) {
        self.b ^= self.c;
        self.instruction_pointer += 1;
    }

    fn out(&mut self, operand: u8) -> u8 {
        let value = (self.combo(operand) % 8) as u8;
        self.instruction_pointer += 1;
        value
    }

    fn bdv(&mut self, operand: u8) {
        self.b = self.a / 2u64.pow(self.combo(operand) as u32);
        self.instruction_pointer += 1;
    }

    fn cdv(&mut self, operand: u8) {
        self.c = self.a / 2u64.pow(self.combo(operand) as u32);
        self.instruction_pointer += 1;
    }
}

impl Iterator for Solver {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_output()
    }
}

struct Output(Vec<u8>);

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        if let Some(&o) = iter.next() {
            write!(f, "{}", o)?;
        }
        for &o in iter {
            write!(f, ",{}", o)?;
        }
        Ok(())
    }
}

impl FromIterator<u8> for Output {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let v = iter.into_iter().collect();
        Self(v)
    }
}
