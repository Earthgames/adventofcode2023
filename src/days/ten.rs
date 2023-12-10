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

    // find main loop
    loop {
        if current_pos == start_pos {
            break;
        }
        clean_grid[current_pos.1][current_pos.0] = 1;

        (current_pos, next_dir) = move_to(current_pos, next_dir, &grid);
    }

    // find a F piece where we know the in and outside
    'big: for (y, row) in clean_grid.iter().enumerate() {
        for (x, num) in row.iter().enumerate() {
            if num == &1 {
                current_pos = (x, y);
                break 'big;
            }
        }
    }
    // loop clockwise from the found F piece and fill on the right side
    let start_pos = current_pos;
    next_dir = East;

    (current_pos, next_dir) = move_to(start_pos, next_dir, &grid);

    loop {
        if current_pos == start_pos {
            break;
        }
        let (x, y) = get_direction(current_pos, next_dir.clockwise());
        if real_direction((x, y), next_dir, &grid) && clean_grid[y][x] == 0 {
            clean_grid[y][x] = 2;
        }

        (current_pos, next_dir) = move_to(current_pos, next_dir, &grid);
    }
    // loop counter clockwise from the found F piece and fill on the left side because
    // **.
    // -7!
    // .|*
    // we forgot to check the !
    let start_pos = current_pos;
    next_dir = South;

    (current_pos, next_dir) = move_to(start_pos, next_dir, &grid);

    loop {
        if current_pos == start_pos {
            break;
        }

        let (x, y) = get_direction(current_pos, next_dir.counterclockwise());
        if real_direction((x, y), next_dir, &grid) && clean_grid[y][x] == 0 {
            clean_grid[y][x] = 2;
        }

        (current_pos, next_dir) = move_to(current_pos, next_dir, &grid);
    }

    // fill all empty space connected to an 2
    for (y, line) in clean_grid.clone().iter().enumerate() {
        for (x, num) in line.iter().enumerate() {
            if num == &2 {
                fill((x, y), &mut clean_grid);
            }
        }
    }

    // count the 2s in the grid, uncomment print to print the map, where 0 is nothing or junk, 2 in inside the main loop, and 1 is the main loop
    let mut total = 0;
    for line in clean_grid {
        for char in line {
            if char == 2 {
                total += 1;
            }
            // print!("{}", char);
        }
        // println!();
    }
    // println!();
    println!("{}", total);
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

    pub fn inverse(&self) -> Direction {
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
    pub fn clockwise(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    pub fn counterclockwise(&self) -> Direction {
        self.clockwise().inverse()
    }
}

fn fill(position: (usize, usize), clean_grid: &mut Vec<Vec<u8>>) {
    for direction in Direction::iterator() {
        let (x, y) = get_direction(position, direction);
        if real_direction(position, direction, clean_grid) && clean_grid[y][x] == 0 {
            clean_grid[y][x] = 2;
            fill((x, y), clean_grid)
        }
    }
}

fn move_to(
    position: (usize, usize),
    direction: Direction,
    grid: &Vec<Vec<[Direction; 2]>>,
) -> ((usize, usize), Direction) {
    if is_valid_dir(position, direction, grid) {
        let (x, y) = get_direction(position, direction);
        let new = grid[y][x];
        return ((x, y), other_direction(direction.inverse(), new));
    }
    (position, direction)
}

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
                    partial_result.push([North, North])
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
    position: (usize, usize),
    direction: Direction,
    grid: &Vec<Vec<[Direction; 2]>>,
) -> bool {
    if real_direction(position, direction, grid) {
        let (x, y) = get_direction(position, direction);
        grid[y][x].contains(&direction.inverse())
    } else {
        false
    }
}

fn get_direction(position: (usize, usize), direction: Direction) -> (usize, usize) {
    match direction {
        North => (position.0, position.1 - 1),
        South => (position.0, position.1 + 1),
        West => (position.0 - 1, position.1),
        East => (position.0 + 1, position.1),
    }
}

fn other_direction(direction: Direction, directions: [Direction; 2]) -> Direction {
    if direction == directions[0] {
        directions[1]
    } else {
        directions[0]
    }
}

fn real_direction<T>(position: (usize, usize), direction: Direction, grid: &Vec<Vec<T>>) -> bool {
    match direction {
        North => position.1 != 0,
        South => position.1 < grid.len() - 1,
        West => position.0 != 0,
        East => position.0 < grid[0].len() - 1,
    }
}
