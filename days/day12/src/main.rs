use std::cell::Cell;

use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver {
    map: Vec<u8>,
    width: i32,
    height: i32,
}

impl Solver {
    fn new(input: &[u8]) -> Self {
        let mut con = Consume::new(input);
        let mut map = Vec::new();
        let mut height = 0;
        while !con.is_empty() {
            let line = con.next_newline();
            let line = &line[..line.len() - 1];
            map.extend_from_slice(line);
            height += 1;
        }
        let width = map.len() as i32 / height;
        Self { map, width, height }
    }

    fn one(&mut self) -> usize {
        let map = Cell::from_mut(&mut *self.map).as_slice_of_cells();
        let mut price = 0;
        let get_map = |pos| get(map, self.width, self.height, pos);
        let mut stack = Vec::new();
        for (row, y) in map.chunks(self.width as usize).zip(0..) {
            for (plot, x) in row.iter().zip(0..) {
                let current = plot.get();
                if current.is_ascii_lowercase() {
                    continue;
                }
                plot.set(current.to_ascii_lowercase());
                stack.push([y, x]);
                let mut area = 1;
                let mut perimeter = 0;
                while let Some([y, x]) = stack.pop() {
                    for neighbor in [[y - 1, x], [y, x + 1], [y + 1, x], [y, x - 1]] {
                        let Some(neighbor_cell) = get_map(neighbor) else {
                            perimeter += 1;
                            continue;
                        };
                        let neighbor_val = neighbor_cell.get();
                        if neighbor_val == current {
                            area += 1;
                            neighbor_cell.set(current.to_ascii_lowercase());
                            stack.push(neighbor);
                            continue;
                        } else if neighbor_val == current.to_ascii_lowercase() {
                            continue;
                        }
                        perimeter += 1;
                    }
                }
                price += area * perimeter;
            }
        }
        price
    }

    fn two(&mut self) -> usize {
        let map = Cell::from_mut(&mut *self.map).as_slice_of_cells();
        let mut price = 0;
        let get_map = |pos| get(map, self.width, self.height, pos);
        let mut stack = Vec::new();

        // Negatives are above or to the left
        let mut verticals = BTreeSet::new();
        let mut horizontals = BTreeSet::new();
        for (row, y) in map.chunks(self.width as usize).zip(0..) {
            for (plot, x) in row.iter().zip(0..) {
                let current = plot.get();
                if current.is_ascii_lowercase() {
                    continue;
                }
                plot.set(current.to_ascii_lowercase());
                stack.push([y, x]);
                let mut area = 1;
                while let Some([y, x]) = stack.pop() {
                    for (neighbor, is_horizontal, addable) in [
                        ([y - 1, x], true, [-y, -x]),
                        ([y, x + 1], false, [x + 1, y]),
                        ([y + 1, x], true, [y + 1, x]),
                        ([y, x - 1], false, [-x, -y]),
                    ] {
                        if let Some(neighbor_cell) = get_map(neighbor) {
                            let neighbor_val = neighbor_cell.get();
                            if neighbor_val == current {
                                area += 1;
                                neighbor_cell.set(current.to_ascii_lowercase());
                                stack.push(neighbor);
                                continue;
                            } else if neighbor_val == current.to_ascii_lowercase() {
                                continue;
                            }
                        }
                        if is_horizontal {
                            horizontals.insert(addable);
                        } else {
                            verticals.insert(addable);
                        }
                    }
                }
                let mut sides = 2;
                // eprintln!("{verticals:?}");
                // eprintln!("{horizontals:?}");
                for set in [&verticals, &horizontals] {
                    let mut last = *set.first().unwrap();
                    for &next in set.iter().skip(1) {
                        if last[0] != next[0] || last[1] != next[1] - 1 {
                            sides += 1;
                        }
                        last = next;
                    }
                }
                let this_price = area * sides;
                // eprintln!("{}: {area} * {sides} = {this_price}", current as char);
                price += this_price;
                verticals.clear();
                horizontals.clear();
            }
        }
        price
    }
}

fn get<T>(map: &[T], width: i32, height: i32, [y, x]: [i32; 2]) -> Option<&T> {
    if !(0..width).contains(&x) || !(0..height).contains(&y) {
        return None;
    }
    Some(&map[(y * width + x) as usize])
}

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).one()
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        Self::new(input).two()
    }
}
