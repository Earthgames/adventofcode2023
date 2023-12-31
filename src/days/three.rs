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
            if column.is_ascii_digit() {
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

pub fn runtwo(input: String) {
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
        for (x, column) in row.iter().enumerate() {
            if column == &'*' {
                let adj = get_adjecent_number(&engine, (x, y));
                if adj.len() == 2 {
                    total += adj[0] * adj[1];
                }
            }
        }
    }
    println!("{}", total);
}

fn get_num(engine: &Vec<Vec<char>>, position: (usize, usize)) -> u32 {
    let mut result = "".to_string();
    if position.0 as i32 > 0 && engine[position.1][position.0 - 1].is_ascii_digit() {
        if position.0 as i32 - 2 >= 0 && engine[position.1][position.0 - 2].is_ascii_digit() {
            result.push(engine[position.1][position.0 - 2])
        }
        result.push(engine[position.1][position.0 - 1]);
    }
    result.push(engine[position.1][position.0]);

    if position.0 < engine[position.1].len()
        && engine[position.1][position.0 + 1].is_ascii_digit()
    {
        result.push(engine[position.1][position.0 + 1]);
        if position.0 + 2 <= engine[position.1].len() && engine[position.1][position.0 + 2].is_ascii_digit() {
            result.push(engine[position.1][position.0 + 2])
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
                        && !engine[cur_pos.1][cur_pos.0].is_ascii_digit()
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn get_adjecent_number(engine: &Vec<Vec<char>>, position: (usize, usize)) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
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
                    if engine[cur_pos.1][cur_pos.0].is_ascii_digit() {
                        result.push(get_num(engine, cur_pos));
                    }
                }
            }
        }
    }
    result.sort();
    result.dedup();
    result
}
