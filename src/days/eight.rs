use num::integer::lcm;
use std::{collections::HashMap, str::Lines};

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

pub fn runtwo(input: String) {
    let mut lines = input.as_str().lines();
    let directions = lines.next().unwrap();
    lines.next();
    let dic = make_node_dic(lines);
    let mut current_nodes: Vec<&str> = vec![];

    for (node, _) in &dic {
        if node.ends_with('A') {
            current_nodes.push(node);
            println!("{}", node);
        }
    }
    let mut total: u64 = 0;
    let mut foundsin = vec![];
    'big: loop {
        for direction in directions.chars() {
            for (i, node) in current_nodes.clone().iter().enumerate() {
                match direction {
                    'R' => current_nodes[i] = &dic[node].right,
                    'L' => current_nodes[i] = &dic[node].left,
                    _ => panic!("here"),
                }
                if current_nodes[i].ends_with('Z') {
                    foundsin.push((current_nodes[i], total + 1));
                }
            }
            total += 1;
            if foundsin.len() == current_nodes.len() {
                break 'big;
            }
        }
    }
    //I stole this form a friend.
    let result = foundsin.into_iter().map(|x| x.1).fold(1, lcm);

    //Below is my attempt at finding the lcm, which is far to slow
    // println!("{:?}", foundsin);
    // let mut totalfounds = foundsin.clone();
    // loop {
    //     totalfounds.sort_by(|a, b| a.1.cmp(&b.1));
    //     for foundin in &foundsin {
    //         if foundin.0 == totalfounds[0].0 {
    //             totalfounds[0].1 += foundin.1;
    //             break;
    //         }
    //     }
    //     let first = totalfounds[0];
    //     if totalfounds.iter().all(|&item| item.1 == first.1) {
    //         break;
    //     }
    // }
    println!("{}", result);
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
