use helpers::*;

fn main() {
    use solver_interface::ChildSolverExt;
    Solver::run().unwrap_display();
}

struct Solver;

impl solver_interface::ChildSolver for Solver {
    fn part_one(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut map = Vec::new();
        let mut con = Consume::new(input);
        let mut row = Vec::new();
        while !con.is_empty() {
            let b = con.consume_byte().unwrap();
            if b == b'\n' {
                map.push(row);
                row = Vec::new();
                continue;
            }
            row.push(b);
        }

        let width = map[0].len();
        let height = map.len();

        let mut count: usize = 0;
        for (y, row) in map.iter().enumerate() {
            for (x, &letter) in row.iter().enumerate() {
                if letter == b'X' {
                    let xmases = [
                        (x <= width - 4).then(|| [map[y][x + 1], map[y][x + 2], map[y][x + 3]]),
                        (x >= 3).then(|| [map[y][x - 1], map[y][x - 2], map[y][x - 3]]),
                        (y <= height - 4).then(|| [map[y + 1][x], map[y + 2][x], map[y + 3][x]]),
                        (y >= 3).then(|| [map[y - 1][x], map[y - 2][x], map[y - 3][x]]),
                        (x <= width - 4 && y <= height - 4)
                            .then(|| [map[y + 1][x + 1], map[y + 2][x + 2], map[y + 3][x + 3]]),
                        (x >= 3 && y >= 3)
                            .then(|| [map[y - 1][x - 1], map[y - 2][x - 2], map[y - 3][x - 3]]),
                        (x <= width - 4 && y >= 3)
                            .then(|| [map[y - 1][x + 1], map[y - 2][x + 2], map[y - 3][x + 3]]),
                        (x >= 3 && y <= height - 4)
                            .then(|| [map[y + 1][x - 1], map[y + 2][x - 2], map[y + 3][x - 3]]),
                    ]
                    .into_iter()
                    .flatten()
                    .filter(|&b| b == *b"MAS")
                    .count();
                    count += xmases;
                }
            }
        }
        count
    }

    fn part_two(input: &[u8], _debug: u8) -> impl Display + 'static {
        let mut map = Vec::new();
        let mut con = Consume::new(input);
        let mut row = Vec::new();
        while !con.is_empty() {
            let b = con.consume_byte().unwrap();
            if b == b'\n' {
                map.push(row);
                row = Vec::new();
                continue;
            }
            row.push(b);
        }

        let width = map[0].len();
        let height = map.len();
        let mut count: usize = 0;
        for (y, row) in map.iter().enumerate().skip(1).take(height - 2) {
            for (x, &letter) in row.iter().enumerate().skip(1).take(width - 2) {
                if letter == b'A' {
                    let corners = [
                        map[y - 1][x - 1],
                        map[y + 1][x + 1],
                        map[y - 1][x + 1],
                        map[y + 1][x - 1],
                    ];
                    if [*b"MSMS", *b"MSSM", *b"SMMS", *b"SMSM"]
                        .into_iter()
                        .any(|p| p == corners)
                    {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}
