pub fn process_part1(input: &str) -> usize {
    let mut sum: usize = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for (i, line) in grid.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'X' {
                // If Up is possible
                if i > 2
                    && check_xmas(
                        Infos {
                            dir: Direction::Up,
                            c: 'M',
                            i,
                            j,
                        },
                        &grid,
                    )
                {
                    sum += 1;
                }
                // If UpRight is possible
                if (i > 2 && j + 3 < grid[i].len())
                    && check_xmas(
                        Infos {
                            dir: Direction::UpRight,
                            c: 'M',
                            i,
                            j,
                        },
                        &grid,
                    )
                {
                    sum += 1;
                }
                // If Right is possible
                if j + 3 < grid[i].len()
                    && check_xmas(
                        Infos {
                            dir: Direction::Right,
                            c: 'M',
                            i,
                            j,
                        },
                        &grid,
                    )
                {
                    sum += 1;
                }
                // If DownRight is possible
                if (i + 3 < grid.len() && j + 3 < grid[i].len())
                    && check_xmas(
                        Infos {
                            dir: Direction::DownRight,
                            c: 'M',
                            i,
                            j,
                        },
                        &grid,
                    )
                {
                    sum += 1;
                }
                // If Down is possible
                if i + 3 < grid.len()
                    && check_xmas(
                        Infos {
                            dir: Direction::Down,
                            c: 'M',
                            i,
                            j,
                        },
                        &grid,
                    )
                {
                    sum += 1;
                }
                // If DownLeft is possible
                if (i + 3 < grid.len() && j > 2)
                    && check_xmas(
                        Infos {
                            dir: Direction::DownLeft,
                            c: 'M',
                            i,
                            j,
                        },
                        &grid,
                    )
                {
                    sum += 1;
                }
                // If Left is possible
                if j > 2
                    && check_xmas(
                        Infos {
                            dir: Direction::Left,
                            c: 'M',
                            i,
                            j,
                        },
                        &grid,
                    )
                {
                    sum += 1;
                }
                // If UpLeft is possible
                if (i > 2 && j > 2)
                    && check_xmas(
                        Infos {
                            dir: Direction::UpLeft,
                            c: 'M',
                            i,
                            j,
                        },
                        &grid,
                    )
                {
                    sum += 1;
                }
                continue;
            }
        }
    }

    sum
}

#[derive(Debug, Clone)]
struct Infos {
    dir: Direction,
    c: char,
    i: usize,
    j: usize,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

fn check_xmas(infos: Infos, grid: &[Vec<char>]) -> bool {
    let cur_i: usize;
    let cur_j: usize;
    match infos.dir {
        Direction::Up => {
            cur_i = infos.i - 1;
            cur_j = infos.j;
        }
        Direction::UpRight => {
            cur_i = infos.i - 1;
            cur_j = infos.j + 1;
        }
        Direction::Right => {
            cur_i = infos.i;
            cur_j = infos.j + 1;
        }
        Direction::DownRight => {
            cur_i = infos.i + 1;
            cur_j = infos.j + 1;
        }
        Direction::Down => {
            cur_i = infos.i + 1;
            cur_j = infos.j;
        }
        Direction::DownLeft => {
            cur_i = infos.i + 1;
            cur_j = infos.j - 1;
        }
        Direction::Left => {
            cur_i = infos.i;
            cur_j = infos.j - 1;
        }
        Direction::UpLeft => {
            cur_i = infos.i - 1;
            cur_j = infos.j - 1;
        }
    };
    if grid[cur_i][cur_j] != infos.c {
        return false;
    }

    match infos.c {
        'S' => true,
        'A' => {
            let mut infos = infos.clone();
            infos.c = 'S';
            infos.i = cur_i;
            infos.j = cur_j;
            check_xmas(infos, grid)
        }
        'M' => {
            let mut infos = infos.clone();
            infos.c = 'A';
            infos.i = cur_i;
            infos.j = cur_j;
            check_xmas(infos, grid)
        }
        _ => false,
    }
}

pub fn process_part2(input: &str) -> usize {
    let mut sum: usize = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for (i, line) in grid.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'A' {
                if i < 1 || i >= grid.len() - 1 || j < 1 || j >= grid[i].len() - 1 {
                    continue;
                }
                if chars(i, j, &grid) {
                    sum += 1;
                }
            }
        }
    }

    sum
}

fn chars(i: usize, j: usize, grid: &[Vec<char>]) -> bool {
    let chars: Vec<char> = vec![
        grid[i - 1][j - 1],
        grid[i - 1][j + 1],
        grid[i + 1][j - 1],
        grid[i + 1][j + 1],
    ];

    chars.iter().filter(|&&c| c == 'M').count() == 2
        && chars.iter().filter(|&&c| c == 'S').count() == 2
        && chars[1] != chars[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    const INPUT2: &str = "\
..X...
.SAMX.
.A..A.
XMAS.S
.X....";

    const INPUT3: &str = "\
MSMMXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_works_1() {
        assert_eq!(process_part1(INPUT1), 18);
    }

    #[test]
    fn part1_works_2() {
        assert_eq!(process_part1(INPUT2), 4);
    }

    #[test]
    fn part2_works_1() {
        assert_eq!(process_part2(INPUT1), 9);
    }

    #[test]
    fn part2_works_2() {
        assert_eq!(process_part2(INPUT3), 8);
    }
}
