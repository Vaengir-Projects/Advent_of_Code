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

pub fn process_part2(_input: &str) -> usize {
    todo!();
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

    #[test]
    fn part1_works_1() {
        assert_eq!(process_part1(INPUT1), 4);
    }

    #[test]
    fn part1_works_2() {
        assert_eq!(process_part1(INPUT2), 8);
    }

    #[test]
    fn part2_works() {
        todo!();
    }
}
