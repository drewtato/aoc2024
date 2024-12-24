#![allow(dead_code)]

use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

#[derive(Debug, Clone)]
struct Solver {
    state: HashMap<Wire, bool>,
    gates: Vec<Gate>,
}

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut solver = Self::new(input);
        solver.sort_gates(&mut HashSet::default());
        solver.simulate().unwrap()
    }
    fn part_two(input: &[u8], debug: u8) -> impl Display + 'static {
        let mut solver = Self::new(input);
        let scratch = &mut HashSet::default();
        solver.sort_gates(scratch);

        let mut bad_gates = Vec::with_capacity(8);

        #[allow(clippy::single_element_loop)]
        for [a, b] in [
            [b"z16", b"hmk"], //
            [b"rvf", b"tpc"],
            [b"fhp", b"z20"],
            [b"fcd", b"z33"],
        ] {
            let a = a.into();
            let b = b.into();
            solver.fix_wires(a, b);
            bad_gates.extend([a, b]);
        }

        // solver.aliases();

        // for gate in solver.gates {
        //     eprintln!("{gate}");
        // }

        if debug == 1 {
            solver.sort_gates(scratch);
            let x = solver.read_register(b'x');
            let y = solver.read_register(b'y');
            let z = solver.simulate().unwrap();
            assert_eq!(x + y, z);
        }

        bad_gates.sort_unstable();
        Output(bad_gates)
    }
}

impl Solver {
    fn new(input: &[u8]) -> Self {
        let mut state = HashMap::default();
        let mut con = Consume::new(input);

        while !con.newline() {
            let line = con.consume(7);
            let &[a, b, c, _, _, data, _] = line else {
                panic!("bad line")
            };
            let data = match data {
                b'0' => false,
                b'1' => true,
                _ => panic!("bad data"),
            };
            state.insert(Wire { bytes: [a, b, c] }, data);
        }

        let mut gates: Vec<Gate> = Vec::new();
        while !con.is_empty() {
            let in1: Wire = con.consume(3).try_into().unwrap();
            con.consume_byte();
            let operation = con.with(|slice| slice.iter().position(|&b| b == b' ').unwrap());
            let operation = match operation {
                b"AND" => And,
                b"OR" => Or,
                b"XOR" => Xor,
                _ => panic!("bad operation"),
            };
            con.consume_byte();
            let in2: Wire = con.consume(3).try_into().unwrap();
            con.consume(4);
            let output: Wire = con.consume(3).try_into().unwrap();

            let gate = Gate {
                in1,
                in2,
                operation,
                output,
            };

            gates.push(gate);
            assert!(con.newline());
        }

        Self { state, gates }
    }

    fn simulate(&mut self) -> Option<u64> {
        self.state.retain(|g, _| matches!(g.bytes[0], b'x' | b'y'));
        for gate in &self.gates {
            // eprintln!("{gate:?}");
            let &in1_bit = self.state.get(&gate.in1)?;
            let &in2_bit = self.state.get(&gate.in2)?;
            let out = gate.operation.apply(in1_bit, in2_bit);
            // eprintln!("{} {:?} {} = {}", gate.in1, gate.operation, gate.in2, out);
            if self.state.insert(gate.output, out).is_some() {
                panic!("two gates updated the same wire");
            }
        }
        Some(self.read_z())
    }

    fn read_register(&self, register: u8) -> u64 {
        let mut wire: Wire = [register, b'0', b'0'].into();
        let mut z = 0;
        let mut index = 0;
        while let Some(&bit) = self.state.get(&wire) {
            // eprintln!("{z:b} {index}");
            z |= (bit as u64) << index;
            wire.increment();
            index += 1;
        }
        z
    }

    fn read_z(&self) -> u64 {
        self.read_register(b'z')
    }

    fn sort_gates(&mut self, scratch_set: &mut HashSet<Wire>) -> bool {
        let seen = scratch_set;
        seen.extend(self.state.keys().copied());
        let mut gates = &mut *self.gates;
        'l: while gates.len() > 1 {
            for (i, &gate) in gates.iter().enumerate() {
                if seen.contains(&gate.in1) && seen.contains(&gate.in2) {
                    gates.swap(0, i);
                    seen.insert(gate.output);
                    gates = &mut gates[1..];
                    continue 'l;
                }
            }
            return false;
        }
        seen.clear();
        true
    }

    fn set_x(&mut self, x: u64) {
        let mut wire: Wire = b"x00".into();
        let mut index = 0;
        while let Some(bit) = self.state.get_mut(&wire) {
            *bit = (x & 1 << index) > 0;
            wire.increment();
            index += 1;
        }
    }

    fn set_y(&mut self, y: u64) {
        let mut wire: Wire = b"y00".into();
        let mut index = 0;
        while let Some(bit) = self.state.get_mut(&wire) {
            *bit = (y & 1 << index) > 0;
            wire.increment();
            index += 1;
        }
    }

    fn swap_outputs(&mut self, a: usize, b: usize) {
        let tmp = self.gates[a].output;
        let a_wire = replace(&mut self.gates[b].output, tmp);
        self.gates[a].output = a_wire;
    }

    fn fix_wires(&mut self, a: Wire, b: Wire) {
        for gate in &mut self.gates {
            if gate.output == a {
                gate.output = b;
            } else if gate.output == b {
                gate.output = a;
            }
        }
    }

    fn aliases(&mut self) {
        let mut aliases: HashMap<Wire, Wire> = HashMap::default();
        for gate in &self.gates {
            let mut args = [gate.in1, gate.in2];
            args.sort_unstable();
            let [x0, x1, x2] = args[0].bytes;
            let [y0, y1, y2] = args[1].bytes;
            if x0 == b'x' && y0 == b'y' && x1 == y1 && x2 == y2 && gate.output.bytes[0] != b'z' {
                match gate.operation {
                    And => aliases.insert(gate.output, [b'A', x1, x2].into()),
                    Xor => aliases.insert(gate.output, [b'E', x1, x2].into()),
                    _ => None,
                };
            }
        }
        for gate in &mut self.gates {
            for wire in [&mut gate.in1, &mut gate.in2, &mut gate.output] {
                if let Some(replacement) = aliases.get(wire) {
                    *wire = *replacement
                }
            }
            if gate.in1 > gate.in2 {
                swap(&mut gate.in1, &mut gate.in2);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Gate {
    in1: Wire,
    in2: Wire,
    operation: Operation,
    output: Wire,
}

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} -> {}",
            self.in1, self.operation, self.in2, self.output
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    And,
    Or,
    Xor,
}
use Operation::*;

impl Operation {
    fn apply(self, a: bool, b: bool) -> bool {
        match self {
            And => a && b,
            Or => a || b,
            Xor => a ^ b,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            And => "AND",
            Or => "OR",
            Xor => "XOR",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Wire {
    bytes: [u8; 3],
}

impl Wire {
    fn increment(&mut self) {
        let mut place = self.bytes.len();
        loop {
            place = place
                .checked_sub(1)
                .expect("wire name overflowed incrementation");
            let digit = &mut self.bytes[place];
            match digit {
                b'0'..=b'8' => *digit += 1,
                b'9' => {
                    *digit = b'0';
                    continue;
                }
                _ => panic!("not a digit: {:?}[{}]", self, self.bytes[place]),
            }
            break;
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for Wire {
    type Error = &'static str;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let &[a, b, c] = value else {
            return Err("slice was not of length 3");
        };
        Ok(Wire { bytes: [a, b, c] })
    }
}

impl From<[u8; 3]> for Wire {
    fn from(value: [u8; 3]) -> Self {
        Self { bytes: value }
    }
}

impl<'a> From<&'a [u8; 3]> for Wire {
    fn from(value: &'a [u8; 3]) -> Self {
        Self { bytes: *value }
    }
}

impl Debug for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", BStr::new(&self.bytes))
    }
}

impl Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BStr::new(&self.bytes))
    }
}
