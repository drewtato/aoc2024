use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

#[derive(Debug, Clone)]
struct Solver {
    disk: Vec<u16>,
}

const SPACE: u16 = u16::MAX;

impl Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &block in &self.disk {
            if block == SPACE {
                write!(f, " .")?;
            } else {
                write!(f, "{block:2}")?;
            }
        }
        Ok(())
    }
}

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        let solver = Self::new(input);

        let mut end = solver.disk.len() - 1;
        let mut total = 0;
        for (i, &block) in solver.disk.iter().enumerate() {
            if i > end {
                break;
            }
            let id = if block == SPACE {
                let block = solver.disk[end];
                end -= 1;
                while solver.disk[end] == SPACE {
                    end -= 1;
                }
                block
            } else {
                block
            };
            // eprintln!("{id} * {i} = {}", id as u64 * i as u64);
            total += id as u64 * i as u64;
        }
        total
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut solver = Self::new(input);

        let space_array = [SPACE; 9];
        // pad out the disk so that there's always a space for any length file.
        solver.disk.extend_from_slice(&space_array);
        let mut first_spaces = [solver.disk.len(); 10];
        let mut index = 0;
        for (len, space) in first_spaces.iter_mut().enumerate().skip(1) {
            loop {
                let disk_slice = &solver.disk[index..index + len];
                if *disk_slice == space_array[..len] {
                    *space = index;
                    break;
                }
                index += 1;
            }
        }

        index = solver.disk.len() - 1;
        let mut id = SPACE;
        while index > 0 {
            // skip spaces and already moved files
            if solver.disk[index] >= id {
                index -= 1;
                continue;
            }
            id = solver.disk[index];

            // find the length of this file
            let mut len = 1;
            for i in (0..index).rev() {
                if solver.disk[i] == id {
                    len += 1;
                    continue;
                }
            }
            // eprintln!("{id} {len} {index}");
            index -= len - 1;

            // get the earliest space for this size of file
            let first_space = first_spaces[len];
            if first_space > index {
                continue;
            }

            // overwrite the earliest space with this file
            for i in (first_space..).take(len) {
                solver.disk[i] = id;
            }

            // overwrite the old file with spaces
            solver.disk[index..][..len].copy_from_slice(&space_array[..len]);

            // recalculate the next earliest space for each affected length
            let mut find_space = first_space + len;
            for (len, space) in first_spaces.iter_mut().enumerate().skip(1) {
                // this space is still available
                if *space < first_space {
                    continue;
                }

                // this and all the rest of the spaces are unaffected
                if *space > first_space {
                    break;
                }

                loop {
                    let disk_slice = &solver.disk[find_space..find_space + len];
                    if *disk_slice == space_array[..len] {
                        *space = find_space;
                        break;
                    }
                    find_space += 1;
                }
            }
        }

        solver
            .disk
            .iter()
            .enumerate()
            .map(
                |(i, &id)| {
                    if id == SPACE { 0 } else { i as u64 * id as u64 }
                },
            )
            .bsum()
    }
}

impl Solver {
    fn new(mut input: &[u8]) -> Self {
        // remove trailing newline
        while input.last().is_some_and(|b| b.is_ascii_whitespace()) {
            input = &input[..input.len() - 1];
        }
        let mut disk = Vec::new();
        for (id, chunk) in input.chunks(2).enumerate() {
            match *chunk {
                [file, space] => {
                    let file = file - b'0';
                    let space = space - b'0';
                    disk.extend(repeat_iter(id as u16).take(file as usize));
                    disk.extend(repeat_iter(SPACE).take(space as usize));
                }
                [file] => {
                    let file = file - b'0';
                    disk.extend(repeat_iter(id as u16).take(file as usize))
                }
                _ => unreachable!(),
            }
        }

        Solver { disk }
    }
}
