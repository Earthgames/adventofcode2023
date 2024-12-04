use regex::Regex;
use SpringStatus::*;

pub fn run(input: String) {
    let spring_records = get_input(input);
    let mut total = 0;
    for spring_record in spring_records {
        let regex = get_regex(spring_record.damage_record);
        total += possibilities(&spring_record.record, &regex);
    }
    println!("{}", total);
}

struct SpringRecord {
    record: Vec<SpringStatus>,
    damage_record: Vec<u16>,
}

#[derive(PartialEq, Clone, Copy)]
enum SpringStatus {
    Unknown,
    Good,
    Damaged,
}

fn possibilities(record: &Vec<SpringStatus>, regex: &Regex) -> u64 {
    let mut current_record = record.clone();
    let mut total = 1;
    for (i, status) in record.iter().enumerate() {
        if status == &Good || status == &Damaged {
            continue;
        }
        let mut found = false;
        current_record.splice(i..i + 1, vec![Damaged]);
        if match_record(&current_record, regex) {
            found = true;
        }
        current_record.splice(i..i + 1, vec![Good]);
        if match_record(&current_record, regex) {
            if found {
                total *= possibilities(&current_record, regex);
            }
        } else if !found {
            return 0;
        }
        if found {
            current_record.splice(i..i + 1, vec![Damaged]);
        }
    }
    total
}

fn match_record(record: &Vec<SpringStatus>, regex: &Regex) -> bool {
    let haystack: String = record
        .iter()
        .map(|c| match c {
            Unknown => '?',
            Good => '.',
            Damaged => '#',
        })
        .collect();
    regex.is_match(&haystack)
}

fn get_regex(damage_record: Vec<u16>) -> Regex {
    let mut regex = String::new();
    // [.?]* == zero or more chars that are . or ?
    //{{{}}} {{ two brackets are to become one bracket in the final string, the last bracket is a bracket to catch the input
    //[?#]{number} == number times ? or #
    regex.push_str(format!(r"[.?]*[?#]{{{}}}", damage_record[0]).as_str());

    for id in 1..damage_record.len() {
        // construct regex
        // [.?]+ == 1 or more chars that are . or ?
        regex.push_str(format!("[.?]+[?#]{{{}}}", damage_record[id]).as_str());
    }
    regex.push_str("[.?]*");
    // println!("{}", regex);
    Regex::new(&regex).unwrap()
}

fn get_input(input: String) -> Vec<SpringRecord> {
    let lines = input.as_str().lines();
    let mut result = Vec::new();
    for line in lines {
        let mut blocks = line.split_whitespace();

        let mut record = Vec::new();
        for char in blocks.next().unwrap().chars() {
            match char {
                '?' => record.push(Unknown),
                '.' => record.push(Good),
                '#' => record.push(Damaged),
                _ => (),
            }
        }

        let mut damage_record = Vec::new();
        for num in blocks.next().unwrap().split(',') {
            damage_record.push(num.parse::<u16>().unwrap());
        }
        result.push(SpringRecord {
            record,
            damage_record,
        })
    }
    result
}
