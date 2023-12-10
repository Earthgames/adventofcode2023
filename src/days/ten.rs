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

pub fn runtwo(input: String) {
    let (grid, start_pos) = prase_input(input);
    let (mut current_pos, mut next_dir) =
        move_to(start_pos, grid[start_pos.1][start_pos.0][0], &grid);
    let mut clean_grid: Vec<Vec<u8>> = vec![vec![0; grid[0].len()]; grid.len()];
    clean_grid[start_pos.1][start_pos.0] = 1;
    loop {
        if current_pos == start_pos {
            break;
        }
        clean_grid[current_pos.1][current_pos.0] = 1;
        (current_pos, next_dir) = move_to(current_pos, next_dir, &grid);
    }

    'big: for (y, row) in clean_grid.iter().enumerate() {
        for (x, num) in row.iter().enumerate() {
            if num == &1 {
                current_pos = (x, y);
                break 'big;
            }
        }
    }
    let start_pos = current_pos;
    next_dir = East;
    (current_pos, next_dir) = move_to(start_pos, next_dir, &grid);
    loop {
        if current_pos == start_pos {
            break;
        }

        match next_dir {
            North => {
                if current_pos.0 < grid[0].len() - 1
                    && clean_grid[current_pos.1][current_pos.0 + 1] == 0
                {
                    clean_grid[current_pos.1][current_pos.0 + 1] = 2;
                }
            }
            South => {
                if current_pos.0 != 0 && clean_grid[current_pos.1][current_pos.0 - 1] == 0 {
                    clean_grid[current_pos.1][current_pos.0 - 1] = 2
                }
            }
            West => {
                if current_pos.1 != 0 && clean_grid[current_pos.1 - 1][current_pos.0] == 0 {
                    clean_grid[current_pos.1 - 1][current_pos.0] = 2;
                }
            }
            East => {
                if current_pos.1 < grid.len() - 1
                    && clean_grid[current_pos.1 + 1][current_pos.0] == 0
                {
                    clean_grid[current_pos.1 + 1][current_pos.0] = 2
                }
            }
        }

        clean_grid[current_pos.1][current_pos.0] = 1;
        (current_pos, next_dir) = move_to(current_pos, next_dir, &grid);
    }
    let start_pos = current_pos;
    next_dir = South;
    (current_pos, next_dir) = move_to(start_pos, next_dir, &grid);
    loop {
        if current_pos == start_pos {
            break;
        }

        match next_dir {
            North => {
                if current_pos.0 != 0 && clean_grid[current_pos.1][current_pos.0 - 1] == 0 {
                    clean_grid[current_pos.1][current_pos.0 - 1] = 2
                }
            }
            South => {
                if current_pos.0 < grid[0].len() - 1
                    && clean_grid[current_pos.1][current_pos.0 + 1] == 0
                {
                    clean_grid[current_pos.1][current_pos.0 + 1] = 2;
                }
            }
            West => {
                if current_pos.1 < grid.len() - 1
                    && clean_grid[current_pos.1 + 1][current_pos.0] == 0
                {
                    clean_grid[current_pos.1 + 1][current_pos.0] = 2
                }
            }
            East => {
                if current_pos.1 != 0 && clean_grid[current_pos.1 - 1][current_pos.0] == 0 {
                    clean_grid[current_pos.1 - 1][current_pos.0] = 2;
                }
            }
        }

        clean_grid[current_pos.1][current_pos.0] = 1;
        (current_pos, next_dir) = move_to(current_pos, next_dir, &grid);
    }
    for (y, line) in clean_grid.clone().iter().enumerate() {
        for (x, num) in line.iter().enumerate() {
            if num == &2 {
                fill((x, y), &mut clean_grid);
            }
        }
    }
    let mut total = 0;
    for line in &clean_grid {
        for char in line {
            if char == &2 {
                total += 1;
            }
            print!("{}", char);
        }
        println!();
    }
    println!();
    println!("{}", total);
}
/*
fn real_direction(
    direction: Direction,
    current_pos: (usize, usize),
    grid: &Vec<Vec<[Direction; 2]>>,
) -> bool {
    match direction {
        North => current_pos.1 != 0,
        South => current_pos.1 < grid.len() - 1,
        West => current_pos.0 != 0,
        East => current_pos.0 < grid[0].len() - 1,
    }
} */

fn fill(current_pos: (usize, usize), clean_grid: &mut Vec<Vec<u8>>) {
    if current_pos.1 != 0 && clean_grid[current_pos.1 - 1][current_pos.0] == 0 {
        clean_grid[current_pos.1 - 1][current_pos.0] = 2;
        fill((current_pos.0, current_pos.1 - 1), clean_grid);
    }
    if current_pos.1 < clean_grid.len() - 1 && clean_grid[current_pos.1 + 1][current_pos.0] == 0 {
        clean_grid[current_pos.1 + 1][current_pos.0] = 2;
        fill((current_pos.0, current_pos.1 + 1), clean_grid);
    }
    if current_pos.0 != 0 && clean_grid[current_pos.1][current_pos.0 - 1] == 0 {
        clean_grid[current_pos.1][current_pos.0 - 1] = 2;
        fill((current_pos.0 - 1, current_pos.1), clean_grid);
    }
    if current_pos.0 < clean_grid[0].len() - 1 && clean_grid[current_pos.1][current_pos.0 + 1] == 0
    {
        clean_grid[current_pos.1][current_pos.0 + 1] = 2;
        fill((current_pos.0 + 1, current_pos.1), clean_grid);
    }
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
        North => {
            if start_pos.1 != 0 {
                return grid[start_pos.1 - 1][start_pos.0].contains(&South);
            }
            false
        }
        South => {
            if start_pos.1 < grid.len() - 1 {
                return grid[start_pos.1 + 1][start_pos.0].contains(&North);
            }
            false
        }
        West => {
            if start_pos.0 != 0 {
                return grid[start_pos.1][start_pos.0 - 1].contains(&East);
            }
            false
        }
        East => {
            if start_pos.0 < grid[0].len() - 1 {
                return grid[start_pos.1][start_pos.0 + 1].contains(&West);
            }
            false
        }
    }
}
