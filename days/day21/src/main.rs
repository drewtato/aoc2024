#![feature(array_windows, let_chains)]
#![allow(clippy::enum_variant_names)]

use std::num::NonZeroU64;

use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

#[derive(Debug, Clone)]
struct Solver {
    codes: Vec<(u64, Vec<Numeric>)>,
}

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).one()
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).two()
    }
}

impl Solver {
    fn new(input: &[u8]) -> Self {
        let mut con = Consume::new(input);
        let mut codes = Vec::new();
        while !con.is_empty() {
            let code = con.next_newline();
            let code = &code[..code.len() - 1];
            let n = parse_ascii(code).unwrap();
            codes.push((
                n,
                code.iter()
                    .copied()
                    .map(Numeric::from_u8)
                    .collect::<Option<_>>()
                    .unwrap(),
            ));
        }
        Self { codes }
    }

    fn one(&self) -> u64 {
        let mut depth = Layer::new_layers(3);
        let ans = self
            .codes
            .iter()
            .map(|&(n, ref buttons)| {
                let length = Self::shortest(buttons, &mut depth);
                length * n
            })
            .sum();
        ans
    }

    fn two(&self) -> u64 {
        let mut depth = Layer::new_layers(26);
        self.codes
            .iter()
            .map(|&(n, ref buttons)| {
                let length = Self::shortest(buttons, &mut depth);
                length * n
            })
            .sum()
    }

    fn shortest(buttons: &[Numeric], depth: &mut [Layer]) -> u64 {
        let mut numeric = NumericA;
        buttons
            .iter()
            .map(|&b| numeric.move_to_and_press(b, depth))
            .sum()
    }
}

const NUMERIC_LAYOUT: [[Numeric; 3]; 4] = [
    [N7, N8, N9],                 //
    [N4, N5, N6],                 //
    [N1, N2, N3],                 //
    [NumericEmpty, N0, NumericA], //
];

const DIRECTIONAL_LAYOUT: [[Directional; 3]; 2] = [
    [DirectionalEmpty, Arrow(North), DirectionalA], //
    [Arrow(West), Arrow(South), Arrow(East)],       //
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Numeric {
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    NumericA,
    NumericEmpty,
}
use Numeric::*;

impl Numeric {
    fn from_u8(b: u8) -> Option<Self> {
        let n = match b {
            b'0' => N0,
            b'1' => N1,
            b'2' => N2,
            b'3' => N3,
            b'4' => N4,
            b'5' => N5,
            b'6' => N6,
            b'7' => N7,
            b'8' => N8,
            b'9' => N9,
            b'A' => NumericA,
            _ => return None,
        };
        Some(n)
    }

    fn to_pos(self) -> [i32; 2] {
        NUMERIC_LAYOUT
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .position(|&n| n == self)
                    .map(|x| [y as i32, x as i32])
            })
            .unwrap()
    }

    fn move_to_and_press(&mut self, b: Self, depth: &mut [Layer]) -> u64 {
        let a = *self;
        *self = b;

        let [ay, ax] = a.to_pos();
        let [by, bx] = b.to_pos();
        let dy = by - ay;
        let dx = bx - ax;

        let vert_horiz = 'b: {
            if ax == 0 && by == 3 {
                break 'b u64::MAX;
            }

            let mut length = 0;
            let mut directional = DirectionalA;

            for _ in 0..dy {
                length += directional.move_to_and_press(South, depth);
            }
            for _ in dy..0 {
                length += directional.move_to_and_press(North, depth);
            }
            for _ in 0..dx {
                length += directional.move_to_and_press(East, depth);
            }
            for _ in dx..0 {
                length += directional.move_to_and_press(West, depth);
            }
            length + directional.move_to_and_press(DirectionalA, depth)
        };

        let horiz_vert = 'b: {
            if ay == 3 && bx == 0 {
                break 'b u64::MAX;
            }

            let mut length = 0;
            let mut directional = DirectionalA;

            for _ in 0..dx {
                length += directional.move_to_and_press(East, depth);
            }
            for _ in dx..0 {
                length += directional.move_to_and_press(West, depth);
            }
            for _ in 0..dy {
                length += directional.move_to_and_press(South, depth);
            }
            for _ in dy..0 {
                length += directional.move_to_and_press(North, depth);
            }
            length + directional.move_to_and_press(DirectionalA, depth)
        };

        vert_horiz.min(horiz_vert)
    }
}

impl Display for Numeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            N0 => '0',
            N1 => '1',
            N2 => '2',
            N3 => '3',
            N4 => '4',
            N5 => '5',
            N6 => '6',
            N7 => '7',
            N8 => '8',
            N9 => '9',
            NumericA => 'A',
            NumericEmpty => '.',
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Directional {
    Arrow(Direction),
    DirectionalA,
    DirectionalEmpty,
}
use Direction::*;
use Directional::*;

impl Directional {
    fn to_pos(self) -> [i32; 2] {
        DIRECTIONAL_LAYOUT
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .position(|&n| n == self)
                    .map(|x| [y as i32, x as i32])
            })
            .unwrap()
    }

    fn to_index(self) -> usize {
        match self {
            Arrow(North) => 0,
            Arrow(East) => 1,
            Arrow(South) => 2,
            Arrow(West) => 3,
            DirectionalA => 4,
            DirectionalEmpty => 5,
        }
    }

    fn move_to_and_press(&mut self, b: impl Into<Directional>, depth: &mut [Layer]) -> u64 {
        let b = b.into();
        let a = *self;
        *self = b;

        let Some((layer, depth)) = depth.split_first_mut() else {
            return 1; // the human pressing the button
        };

        layer.get_or_insert_with(a, b, || {
            let [ay, ax] = a.to_pos();
            let [by, bx] = b.to_pos();
            let dy = by - ay;
            let dx = bx - ax;

            let vert_horiz = 'b: {
                if ax == 0 && by == 0 {
                    break 'b u64::MAX;
                }

                let mut length = 0;
                let mut directional = DirectionalA;

                for _ in 0..dy {
                    length += directional.move_to_and_press(South, depth);
                }
                for _ in dy..0 {
                    length += directional.move_to_and_press(North, depth);
                }

                for _ in 0..dx {
                    length += directional.move_to_and_press(East, depth);
                }
                for _ in dx..0 {
                    length += directional.move_to_and_press(West, depth);
                }
                length + directional.move_to_and_press(DirectionalA, depth)
            };

            let horiz_vert = 'b: {
                if ay == 0 && bx == 0 {
                    break 'b u64::MAX;
                }

                let mut length = 0;
                let mut directional = DirectionalA;

                for _ in 0..dx {
                    length += directional.move_to_and_press(East, depth);
                }
                for _ in dx..0 {
                    length += directional.move_to_and_press(West, depth);
                }

                for _ in 0..dy {
                    length += directional.move_to_and_press(South, depth);
                }
                for _ in dy..0 {
                    length += directional.move_to_and_press(North, depth);
                }

                length + directional.move_to_and_press(DirectionalA, depth)
            };

            vert_horiz.min(horiz_vert)
        })
    }
}

impl From<Direction> for Directional {
    fn from(value: Direction) -> Self {
        Arrow(value)
    }
}

impl Display for Directional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Arrow(North) => '^',
            Arrow(East) => '>',
            Arrow(South) => 'v',
            Arrow(West) => '<',
            DirectionalA => 'A',
            DirectionalEmpty => '.',
        })
    }
}

#[derive(Debug, Clone, Default)]
struct Layer {
    memo: [[Option<NonZeroU64>; 5]; 5],
}

impl Layer {
    fn new_layers(depth: usize) -> Vec<Self> {
        (0..depth - 1).map(|_| Self::default()).collect()
    }

    fn get_or_insert_with(
        &mut self,
        a: Directional,
        b: Directional,
        f: impl FnOnce() -> u64,
    ) -> u64 {
        self.memo[a.to_index()][b.to_index()]
            .get_or_insert_with(|| {
                let ans = f();
                ans.try_into().unwrap()
            })
            .get()
    }
}
