use self::Direction::*;

pub fn run(input: String) {
    let (grid, start_pos) = prase_input(input);
    let (mut current_pos, mut next_dir) =
        move_to(start_pos, grid[start_pos.1][start_pos.0][0], &grid);
    let mut total: u64 = 1;
    loop {
        if current_pos == start_pos {
            break;
        }
        (current_pos, next_dir) = move_to(current_pos, next_dir, &grid);
        total += 1;
    }
    println!("{}", total / 2);
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn iterator() -> impl Iterator<Item = Direction> {
        [North, South, East, West].iter().copied()
    }
}

fn move_to(
    position: (usize, usize),
    direction: Direction,
    grid: &Vec<Vec<[Direction; 2]>>,
) -> ((usize, usize), Direction) {
    if is_valid_dir(position, direction, grid) {
        match direction {
            North => {
                let return_direction = if grid[position.1 - 1][position.0][0] == South {
                    grid[position.1 - 1][position.0][1]
                } else {
                    grid[position.1 - 1][position.0][0]
                };
                return ((position.0, position.1 - 1), return_direction);
            }
            South => {
                let return_direction = if grid[position.1 + 1][position.0][0] == North {
                    grid[position.1 + 1][position.0][1]
                } else {
                    grid[position.1 + 1][position.0][0]
                };
                return ((position.0, position.1 + 1), return_direction);
            }
            West => {
                let return_direction = if grid[position.1][position.0 - 1][0] == East {
                    grid[position.1][position.0 - 1][1]
                } else {
                    grid[position.1][position.0 - 1][0]
                };
                return ((position.0 - 1, position.1), return_direction);
            }
            East => {
                let return_direction = if grid[position.1][position.0 + 1][0] == West {
                    grid[position.1][position.0 + 1][1]
                } else {
                    grid[position.1][position.0 + 1][0]
                };
                return ((position.0 + 1, position.1), return_direction);
            }
        }
    }
    (position, direction)
}

// true is right or up
fn prase_input(input: String) -> (Vec<Vec<[Direction; 2]>>, (usize, usize)) {
    let lines = input.as_str().lines();
    let mut result: Vec<Vec<[Direction; 2]>> = vec![];
    let mut start_pos = (0, 0);
    for (y, line) in lines.enumerate() {
        let mut partial_result: Vec<[Direction; 2]> = vec![];
        for (x, char) in line.chars().enumerate() {
            match char {
                '|' => partial_result.push([North, South]),
                '-' => partial_result.push([East, West]),
                'L' => partial_result.push([North, East]),
                'J' => partial_result.push([North, West]),
                '7' => partial_result.push([South, West]),
                'F' => partial_result.push([South, East]),
                'S' => {
                    start_pos = (x, y);
                    partial_result.push([North, Direction::North])
                }
                _ => partial_result.push([North, North]),
            }
        }
        result.push(partial_result);
    }
    let mut start_dir: Vec<Direction> = vec![];
    for direction in Direction::iterator() {
        if is_valid_dir(start_pos, direction, &result) {
            start_dir.push(direction);
        }
    }
    result[start_pos.1][start_pos.0] = start_dir.try_into().unwrap();
    (result, start_pos)
}

fn is_valid_dir(
    start_pos: (usize, usize),
    direction: Direction,
    grid: &Vec<Vec<[Direction; 2]>>,
) -> bool {
    match direction {
        Direction::North => {
            if start_pos.1 != 0 {
                return grid[start_pos.1 - 1][start_pos.0].contains(&South);
            }
            return false;
        }
        Direction::South => {
            if start_pos.1 < grid.len() - 1 {
                return grid[start_pos.1 + 1][start_pos.0].contains(&North);
            }
            return false;
        }
        Direction::West => {
            if start_pos.0 != 0 {
                return grid[start_pos.1][start_pos.0 - 1].contains(&East);
            }
            return false;
        }
        Direction::East => {
            if start_pos.0 < grid[0].len() - 1 {
                return grid[start_pos.1][start_pos.0 + 1].contains(&West);
            }
            return false;
        }
    }
}
