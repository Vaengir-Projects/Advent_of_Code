#[derive(Debug, Clone, Copy)]
struct Pipe {
    pipe: PipeType,
    distance: usize,
    index: (usize, usize),
    path_to_here: Direction,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum PipeType {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    fn iterator() -> impl Iterator<Item = Direction> {
        [
            Direction::Top,
            Direction::Right,
            Direction::Bottom,
            Direction::Left,
        ]
        .iter()
        .copied()
    }
}

fn check_adjacent(
    matrix: &[Vec<PipeType>],
    row: i32,
    column: i32,
    direction: Direction,
) -> Option<Pipe> {
    let (row, column) = match direction {
        Direction::Top => (row - 1, column),
        Direction::Right => (row, column + 1),
        Direction::Bottom => (row + 1, column),
        Direction::Left => (row, column - 1),
    };
    if let Some(row_vec) = matrix.get(row as usize) {
        if let Some(element) = row_vec.get(column as usize) {
            return Some(Pipe {
                pipe: *element,
                distance: 1,
                index: (row.try_into().unwrap(), column.try_into().unwrap()),
                path_to_here: direction,
            });
        }
    }
    None
}

fn loop_around(pipe: &Pipe, matrix: &Vec<Vec<PipeType>>) -> usize {
    if let PipeType::Start = pipe.pipe {
        return pipe.distance / 2;
    }
    let ((next_row, next_column), next_path) = match (pipe.path_to_here, pipe.pipe) {
        (Direction::Top, PipeType::NS) => ((pipe.index.0 - 1, pipe.index.1), Direction::Top),
        (Direction::Top, PipeType::SW) => ((pipe.index.0, pipe.index.1 - 1), Direction::Left),
        (Direction::Top, PipeType::SE) => ((pipe.index.0, pipe.index.1 + 1), Direction::Right),
        (Direction::Right, PipeType::NW) => ((pipe.index.0 - 1, pipe.index.1), Direction::Top),
        (Direction::Right, PipeType::SW) => ((pipe.index.0 + 1, pipe.index.1), Direction::Bottom),
        (Direction::Right, PipeType::EW) => ((pipe.index.0, pipe.index.1 + 1), Direction::Right),
        (Direction::Bottom, PipeType::NS) => ((pipe.index.0 + 1, pipe.index.1), Direction::Bottom),
        (Direction::Bottom, PipeType::NW) => ((pipe.index.0, pipe.index.1 - 1), Direction::Left),
        (Direction::Bottom, PipeType::NE) => ((pipe.index.0, pipe.index.1 + 1), Direction::Right),
        (Direction::Left, PipeType::NE) => ((pipe.index.0 - 1, pipe.index.1), Direction::Top),
        (Direction::Left, PipeType::EW) => ((pipe.index.0, pipe.index.1 - 1), Direction::Left),
        (Direction::Left, PipeType::SE) => ((pipe.index.0 + 1, pipe.index.1), Direction::Bottom),
        _ => panic!("Not a valid combination of path_to_here and PipeType"),
    };
    let next = Pipe {
        pipe: matrix[next_row][next_column],
        distance: pipe.distance + 1,
        index: (next_row, next_column),
        path_to_here: next_path,
    };
    loop_around(&next, matrix)
}

pub fn process_part1(input: &str) -> usize {
    let matrix: Vec<Vec<PipeType>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => PipeType::NS,
                    '-' => PipeType::EW,
                    'L' => PipeType::NE,
                    'J' => PipeType::NW,
                    '7' => PipeType::SW,
                    'F' => PipeType::SE,
                    '.' => PipeType::Ground,
                    'S' => PipeType::Start,
                    _ => panic!("Invalid char while parsing matrix"),
                })
                .collect()
        })
        .collect();
    let (start_row, start_column) = matrix
        .iter()
        .enumerate()
        .find_map(|(row_index, row)| {
            row.iter()
                .position(|element| *element == PipeType::Start)
                .map(|col_index| (row_index, col_index))
        })
        .unwrap();
    let top = [PipeType::NS, PipeType::SW, PipeType::SE];
    let right = [PipeType::EW, PipeType::NW, PipeType::SW];
    let bottom = [PipeType::NS, PipeType::SW, PipeType::SE];
    let left = [PipeType::EW, PipeType::SE, PipeType::NE];
    let mut adjacent: Vec<Pipe> = Vec::new();
    for direction in Direction::iterator() {
        if let Some(element) = check_adjacent(
            &matrix,
            start_row.try_into().unwrap(),
            start_column.try_into().unwrap(),
            direction,
        ) {
            if (direction == Direction::Top && top.contains(&element.pipe))
                || (direction == Direction::Right && right.contains(&element.pipe))
                || (direction == Direction::Bottom && bottom.contains(&element.pipe))
                || (direction == Direction::Left && left.contains(&element.pipe))
            {
                adjacent.push(element);
            }
        }
    }
    let first: Pipe = adjacent[0];
    loop_around(&first, &matrix)
}

fn build_pipe(matrix: &mut [Vec<char>], mut row: usize, mut column: usize, mut dir: Direction) {
    while matrix[row][column] != 'S' {
        if dir == Direction::Top {
            match matrix[row][column] {
                '|' => {
                    matrix[row][column] = 'P';
                    row -= 1;
                    dir = Direction::Top;
                }
                '7' => {
                    matrix[row][column] = 'P';
                    column -= 1;
                    dir = Direction::Left;
                }
                'F' => {
                    matrix[row][column] = 'P';
                    column += 1;
                    dir = Direction::Right;
                }
                _ => panic!("Error while matching: North"),
            }
        } else if dir == Direction::Right {
            match matrix[row][column] {
                '-' => {
                    matrix[row][column] = 'P';
                    column += 1;
                    dir = Direction::Right;
                }
                '7' => {
                    matrix[row][column] = 'P';
                    row += 1;
                    dir = Direction::Bottom;
                }
                'J' => {
                    matrix[row][column] = 'P';
                    row -= 1;
                    dir = Direction::Top;
                }
                _ => panic!("Error while matching: East"),
            }
        } else if dir == Direction::Bottom {
            match matrix[row][column] {
                '|' => {
                    matrix[row][column] = 'P';
                    row += 1;
                    dir = Direction::Bottom;
                }
                'L' => {
                    matrix[row][column] = 'P';
                    column += 1;
                    dir = Direction::Right;
                }
                'J' => {
                    matrix[row][column] = 'P';
                    column -= 1;
                    dir = Direction::Left;
                }
                _ => {
                    panic!("Error while matching: South");
                }
            }
        } else if dir == Direction::Left {
            match matrix[row][column] {
                '-' => {
                    matrix[row][column] = 'P';
                    column -= 1;
                    dir = Direction::Left;
                }
                'L' => {
                    matrix[row][column] = 'P';
                    row -= 1;
                    dir = Direction::Top;
                }
                'F' => {
                    matrix[row][column] = 'P';
                    row += 1;
                    dir = Direction::Bottom;
                }
                _ => panic!("Error while matching: West"),
            }
        }
    }
}

fn set_edge(matrix: &mut [Vec<char>]) {
    for col_nr in 0..matrix[0].len() {
        matrix[0][col_nr] = 'E';
        matrix[matrix.len() - 1][col_nr] = 'E';
    }
    let matrix_row_len = matrix[0].len() - 1;
    for row_nr in 0..matrix.len() {
        matrix[row_nr][0] = 'E';
        matrix[row_nr][matrix_row_len] = 'E';
    }
}

fn get_adjacent(matrix: &mut [Vec<char>]) {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'E' {
                // Iterate over adjacent cells (within a 3x3 square)
                for di in -1..=1 {
                    for dj in -1..=1 {
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        // Check if the adjacent cell is within bounds and not the current cell
                        if ni >= 0
                            && ni < matrix.len() as isize
                            && nj >= 0
                            && nj < matrix[i].len() as isize
                            && !(di == 0 && dj == 0)
                            && matrix[ni as usize][nj as usize] != 'E'
                            && matrix[ni as usize][nj as usize] != 'P'
                            && matrix[ni as usize][nj as usize] != 'S'
                        {
                            matrix[ni as usize][nj as usize] = 'E';
                        }
                    }
                }
            }
        }
    }
}

pub fn process_part2(input: &str) -> usize {
    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (mut row, mut column) = matrix
        .iter()
        .enumerate()
        .find_map(|(row_index, row)| {
            row.iter()
                .position(|element| *element == 'S')
                .map(|col_index| (row_index, col_index))
        })
        .unwrap();
    let dir: Direction;
    if matrix[row - 1][column] == '|'
        || matrix[row - 1][column] == '7'
        || matrix[row - 1][column] == 'F'
    {
        row -= 1;
        match matrix[row][column] {
            '|' => {
                matrix[row][column] = 'P';
                row -= 1;
                dir = Direction::Top;
            }
            '7' => {
                matrix[row][column] = 'P';
                column -= 1;
                dir = Direction::Left;
            }
            'F' => {
                matrix[row][column] = 'P';
                column += 1;
                dir = Direction::Right;
            }
            _ => panic!("Error while matching start: North"),
        }
    } else if matrix[row][column + 1] == '-'
        || matrix[row][column + 1] == '7'
        || matrix[row][column + 1] == 'J'
    {
        column += 1;
        match matrix[row][column] {
            '-' => {
                matrix[row][column] = 'P';
                column += 1;
                dir = Direction::Right;
            }
            '7' => {
                matrix[row][column] = 'P';
                row += 1;
                dir = Direction::Bottom;
            }
            'J' => {
                matrix[row][column] = 'P';
                row -= 1;
                dir = Direction::Top;
            }
            _ => panic!("Error while matching start: East"),
        }
    } else if matrix[row + 1][column] == '|'
        || matrix[row + 1][column] == 'L'
        || matrix[row + 1][column] == 'J'
    {
        column += 1;
        match matrix[row][column] {
            '|' => {
                matrix[row][column] = 'P';
                row += 1;
                dir = Direction::Bottom;
            }
            'L' => {
                matrix[row][column] = 'P';
                column += 1;
                dir = Direction::Right;
            }
            'J' => {
                matrix[row][column] = 'P';
                column -= 1;
                dir = Direction::Left;
            }
            _ => panic!("Error while matching start: South"),
        }
    } else if matrix[row][column - 1] == '-'
        || matrix[row][column - 1] == 'L'
        || matrix[row][column - 1] == 'F'
    {
        column += 1;
        match matrix[row][column] {
            '-' => {
                matrix[row][column] = 'P';
                row -= 1;
                dir = Direction::Left;
            }
            'L' => {
                matrix[row][column] = 'P';
                row -= 1;
                dir = Direction::Top;
            }
            'F' => {
                matrix[row][column] = 'P';
                row += 1;
                dir = Direction::Bottom;
            }
            _ => panic!("Error while matching start: West"),
        }
    } else {
        panic!("Dir not initialized");
    }
    build_pipe(&mut matrix, row, column, dir);
    set_edge(&mut matrix);
    get_adjacent(&mut matrix);
    for row in &matrix {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
    matrix
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c != 'S' && c != 'P' && c != 'E')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";

    const INPUT2: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const INPUT3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const INPUT4: &str = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    const INPUT5: &str = "\
......................
..F----7F7F7F7F-7.....
..|F--7||||||||FJ.....
..||.FJ||||||||L7.....
.FJL7L7LJLJ||LJ.L-7...
.L--J.L7...LJS7F-7L7..
.....F-J..F7FJ|L7L7L7.
.....L7.F7||L7|.L7L7|.
......|FJLJ|FJ|F7|.LJ.
.....FJL-7.||.||||....
.....L---J.LJ.LJLJ....
......................";

    #[test]
    fn part1_works_1() {
        assert_eq!(process_part1(INPUT1), 4);
    }

    #[test]
    fn part1_works_2() {
        assert_eq!(process_part1(INPUT2), 8);
    }

    #[test]
    fn part2_works_1() {
        assert_eq!(process_part2(INPUT3), 4);
    }

    #[test]
    fn part2_works_2() {
        assert_eq!(process_part2(INPUT4), 4);
    }

    #[test]
    fn part2_works_3() {
        assert_eq!(process_part2(INPUT5), 8);
    }
}
