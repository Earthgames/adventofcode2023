use std::{collections::HashMap, str::Lines};

use super::four;

pub fn run(input: String) {
    let mut lines = input.as_str().lines();
    let directions = lines.next().unwrap();
    lines.next();
    let dic = make_node_dic(lines);
    let mut current_node = String::from("AAA");
    let mut total: u64 = 0;
    loop {
        let (count, found, current_out) = search(current_node.as_str(), &dic, directions);
        total += count;
        current_node = current_out;
        if found {
            break;
        }
    }
    println!("{total}")
}

fn search(current_node: &str, dic: &HashMap<&str, Node>, directions: &str) -> (u64, bool, String) {
    let mut count = 0;
    let mut found = false;
    let mut current_node = current_node;
    for direction in directions.chars() {
        match direction {
            'R' => {
                count += 1;
                current_node = dic[current_node].right.as_str()
            }
            'L' => {
                count += 1;
                current_node = dic[current_node].left.as_str()
            }
            _ => panic!("here"),
        }
        if current_node == "ZZZ" {
            found = true;
            break;
        }
    }
    (count, found, current_node.to_string())
}

struct Node {
    left: String,
    right: String,
}

fn make_node_dic(lines: Lines) -> HashMap<&str, Node> {
    let mut dic: HashMap<&str, Node> = HashMap::new();
    for line in lines {
        let mut parts = line.split(" =");
        let key = parts.next().unwrap();
        let mut directions = parts.next().unwrap().split(',');
        let left = directions
            .next()
            .unwrap()
            .strip_prefix(" (")
            .unwrap()
            .to_string();
        let right = directions
            .next()
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .strip_prefix(' ')
            .unwrap()
            .to_string();
        dic.insert(key, Node { left, right });
    }
    dic
}
