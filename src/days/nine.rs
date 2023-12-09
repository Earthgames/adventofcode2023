use std::str::Lines;

pub fn run(input: String) {
    let lines = input.as_str().lines();
    let sets = get_sets(lines);
    let mut result = 0;
    for set in sets {
        result += newton_forward_polynomial(set.len() as i128, &set)
    }
    println!("{}", result);
}

pub fn runtwo(input: String) {
    let lines = input.as_str().lines();
    let sets = get_sets(lines);
    let mut result = 0;
    for set in sets {
        result += newton_forward_polynomial(-1, &set)
    }
    println!("{}", result);
}

fn get_sets(lines: Lines) -> Vec<Vec<i128>> {
    let mut sets: Vec<Vec<i128>> = vec![];
    for line in lines {
        let mut set: Vec<i128> = vec![];
        for num in line.split_ascii_whitespace() {
            set.push(num.parse::<i128>().unwrap());
        }
        sets.push(set);
    }
    sets
}

// the steps between x are all equal to one
fn newton_forward_polynomial(x: i128, yset: &Vec<i128>) -> i128 {
    let table = get_table(yset);
    let mut result = yset[0];
    let mut upside = 1i128;
    for (i, delta_y) in table.iter().enumerate() {
        upside *= x - i as i128;
        result += delta_y * (upside / factorial(i as u128 + 1) as i128)
    }
    return result;

    fn get_table(table: &Vec<i128>) -> Vec<i128> {
        let mut result = vec![];
        for i in 1..table.len() {
            // println!("{} - {}", table[i], table[i - 1]);
            result.push(table[i] - table[i - 1]);
        }
        if result.len() == 1 || result.iter().all(|x| x == &0) {
            return result;
        }
        let mut reallresult = get_table(&result);
        reallresult.insert(0, result[0]);
        reallresult
    }
}

pub fn factorial(num: u128) -> u128 {
    (1..=num).product()
}
