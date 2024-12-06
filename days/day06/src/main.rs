use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver;

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut con = Consume::new(input);
        let mut map = Vec::new();

        while !con.is_empty() {
            let row = con.next_newline();
            map.push(row[..row.len() - 1].to_vec());
        }

        let mut pos = map
            .iter_mut()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter_mut()
                    .position(|c| {
                        if *c == b'^' {
                            *c = b'.';
                            true
                        } else {
                            false
                        }
                    })
                    .map(|x| [y as i32, x as i32])
            })
            .unwrap();

        let mut dir = [-1, 0];
        let mut visited = HashSet::from_iter([(pos, dir)]);
        loop {
            let next_pos = [pos[0] + dir[0], pos[1] + dir[1]];
            let Some(c) = map
                .get(next_pos[0] as usize)
                .and_then(|row| row.get(next_pos[1] as usize))
                .copied()
            else {
                break;
            };
            if c != b'.' {
                dir = match dir {
                    [-1, 0] => [0, 1],
                    [0, 1] => [1, 0],
                    [1, 0] => [0, -1],
                    [0, -1] => [-1, 0],
                    _ => unreachable!(),
                };
            } else {
                pos = next_pos;
                if !visited.insert((pos, dir)) {
                    panic!("the guard looped already");
                }
            }
        }

        let visited_directionless = visited
            .into_iter()
            .map(|(pos, _)| pos)
            .collect::<HashSet<_>>();

        // for (y, row) in map.iter().enumerate() {
        //     for (x, &c) in row.iter().enumerate() {
        //         if visited_directionless.contains(&[y as i32, x as i32]) {
        //             eprint!("X");
        //         } else {
        //             eprint!("{}", c as char);
        //         }
        //     }
        //     eprintln!();
        // }

        visited_directionless.len()
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut con = Consume::new(input);
        let mut map = Vec::new();

        while !con.is_empty() {
            let row = con.next_newline();
            map.push(row[..row.len() - 1].to_vec());
        }

        let start_pos = map
            .iter_mut()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter_mut()
                    .position(|c| {
                        if *c == b'^' {
                            *c = b'.';
                            true
                        } else {
                            false
                        }
                    })
                    .map(|x| [y as i32, x as i32])
            })
            .unwrap();

        let mut visited = HashSet::default();
        let mut count = 0;
        for row in 0..map.len() {
            for col in 0..map[0].len() {
                if map[row][col] != b'.' {
                    continue;
                }
                map[row][col] = b'#';
                let mut dir = [-1, 0];
                let mut pos = start_pos;
                visited.insert((pos, dir));

                loop {
                    let next_pos = [pos[0] + dir[0], pos[1] + dir[1]];
                    let Some(c) = map
                        .get(next_pos[0] as usize)
                        .and_then(|row| row.get(next_pos[1] as usize))
                        .copied()
                    else {
                        break;
                    };
                    if c != b'.' {
                        dir = match dir {
                            [-1, 0] => [0, 1],
                            [0, 1] => [1, 0],
                            [1, 0] => [0, -1],
                            [0, -1] => [-1, 0],
                            _ => unreachable!(),
                        };
                    } else {
                        pos = next_pos;
                        if !visited.insert((pos, dir)) {
                            count += 1;
                            break;
                        }
                    }
                }

                map[row][col] = b'.';
                visited.clear();
            }
        }

        count
    }
}
