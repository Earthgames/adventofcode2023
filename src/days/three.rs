use std::usize;

pub fn run(input: String) {
    let lines = input.as_str().lines();
    let mut engine: Vec<Vec<char>> =
        vec![vec!['.'; lines.clone().next().unwrap().len()]; lines.clone().count()];
    let mut total: u32 = 0;

    // create 2d vector
    for (y, line) in lines.enumerate() {
        for (x, character) in line.chars().enumerate() {
            engine[y][x] = character;
        }
    }

    for (y, row) in engine.iter().enumerate() {
        let mut not_checked = true;
        for (x, column) in row.iter().enumerate() {
            if column.is_digit(10) {
                if not_checked && is_adjecent(&engine, (x, y)) {
                    let num = get_num(&engine, (x, y));
                    total += num;
                    not_checked = false;
                }
            } else {
                not_checked = true;
            }
        }
    }
    println!("{}", total);
}

fn get_num(engine: &Vec<Vec<char>>, position: (usize, usize)) -> u32 {
    let mut result = "".to_string();
    if !(position.0 as i32 - 1 < 0) && engine[position.1][position.0 - 1].is_digit(10) {
        if !(position.0 as i32 - 2 < 0) {
            if engine[position.1][position.0 - 2].is_digit(10) {
                result.push(engine[position.1][position.0 - 2])
            }
        }
        result.push(engine[position.1][position.0 - 1]);
    }
    result.push(engine[position.1][position.0]);

    if !(position.0 + 1 > engine[position.1].len())
        && engine[position.1][position.0 + 1].is_digit(10)
    {
        result.push(engine[position.1][position.0 + 1]);
        if !(position.0 + 2 > engine[position.1].len()) {
            if engine[position.1][position.0 + 2].is_digit(10) {
                result.push(engine[position.1][position.0 + 2])
            }
        }
    }
    result.parse::<u32>().unwrap()
}

fn is_adjecent(engine: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    for y in -1..=1 {
        if position.1 as i32 + y >= 0 && position.1 as i32 + y < engine.len() as i32 {
            for x in -1..=1 {
                if position.0 as i32 + x >= 0
                    && position.0 as i32 + x < engine[(position.1 as i32 + y) as usize].len() as i32
                {
                    let cur_pos: (usize, usize) = (
                        (position.0 as i32 + x) as usize,
                        (position.1 as i32 + y) as usize,
                    );
                    if engine[cur_pos.1][cur_pos.0] != '.'
                        && !(engine[cur_pos.1][cur_pos.0].is_digit(10))
                    {
                        return true;
                    }
                }
            }
        }
    }
    return false;
}
