use std::{
    cmp::{max, min},
    ops::Range,
};

pub fn run(input: String) {
    let mut blocks = input.split("\n\n");
    let mut seeds: Vec<u64> = vec![];

    for seed in blocks
        .next()
        .unwrap()
        .split(':')
        .next_back()
        .unwrap()
        .split_ascii_whitespace()
    {
        seeds.push(match seed.parse::<u64>() {
            Ok(t) => t,
            Err(_) => continue,
        })
    }

    let mut maps: Vec<Vec<RangeConv>> = vec![];
    for block in blocks {
        maps.push(make_map(block));
    }
    let mut final_num: Vec<u64> = vec![];
    for number in seeds {
        let mut current_num = number;
        for map in &maps {
            current_num = convert_with_map(map, current_num);
        }
        final_num.push(current_num);
    }
    final_num.sort();
    println!("{:?}", final_num[0]);
}

pub fn runtwo(input: String) {
    let mut blocks = input.split("\n\n");
    let mut seeds: Vec<Range<u64>> = vec![];

    let mut seed_str = blocks
        .next()
        .unwrap()
        .split(':')
        .next_back()
        .unwrap()
        .split_ascii_whitespace();

    while let Some(s) = seed_str.next() {
        let start = s.parse::<u64>().unwrap();

        seeds.push(Range {
            start,
            end: start + seed_str.next().unwrap().parse::<u64>().unwrap() - 1,
        })
    }

    let mut maps: Vec<Vec<RangeConv>> = vec![];
    for block in blocks {
        maps.push(make_map(block));
    }

    let mut final_num: Vec<u64> = vec![];
    for number in seeds {
        let mut current_ranges = vec![number];
        for map in &maps {
            current_ranges = convert_range_with_map(map, current_ranges);
        }
        current_ranges.iter().for_each(|r| final_num.push(r.start))
    }
    final_num.sort();
    println!("{:?}", final_num[0]);
}

#[derive(Debug)]
struct RangeConv {
    source_start: u64,
    destination_start: u64,
    size: u64,
}

fn convert_range_with_map(map: &Vec<RangeConv>, ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    let mut result: Vec<Range<u64>> = vec![];
    for range in ranges {
        let mut checked: Vec<Range<u64>> = vec![];
        for rangeconv in map {
            match range_intersect(
                &range,
                &Range {
                    start: rangeconv.source_start,
                    end: rangeconv.source_start + rangeconv.size,
                },
            ) {
                Some(r) => {
                    result.push(Range {
                        start: r.start + rangeconv.destination_start - rangeconv.source_start,
                        end: r.end + rangeconv.destination_start - rangeconv.source_start,
                    });
                    checked.push(r)
                }
                None => continue,
            }
        }
        let mut unchecked = range;

        for range in &checked {
            match range_remove(range, &unchecked) {
                Some(r) => {
                    if r.0.start != r.0.end {
                        result.push(r.0);
                    }
                    unchecked = r.1;
                }
                None => continue,
            }
        }
        if unchecked.start != unchecked.end {
            result.push(unchecked);
        }
    }
    result
}

fn range_intersect(v1: &Range<u64>, v2: &Range<u64>) -> Option<Range<u64>> {
    if v1.start < v2.end && v1.end > v2.start {
        let start = max(v1.start, v2.start);
        let end = min(v1.end, v2.end);
        return Some(Range { start, end });
    }
    None
}

fn range_remove(v1: &Range<u64>, v2: &Range<u64>) -> Option<(Range<u64>, Range<u64>)> {
    if v1.start < v2.end && v1.end > v2.start {
        let start = max(v1.start, v2.start);
        let end = min(v2.end, v2.end);
        return Some((
            Range {
                start: v1.start,
                end: start,
            },
            Range {
                start: end,
                end: v2.end,
            },
        ));
    }
    None
}

fn convert_with_map(map: &Vec<RangeConv>, source_number: u64) -> u64 {
    for range in map {
        match range_number(range, source_number) {
            Some(num) => return num,
            None => continue,
        }
    }
    source_number
}

fn range_number(range: &RangeConv, source_number: u64) -> Option<u64> {
    if source_number < range.source_start || source_number > range.source_start + range.size - 1 {
        return None;
    }
    Some(source_number + range.destination_start - range.source_start)
}

fn make_range(input: &str) -> RangeConv {
    let mut numbers = input.split_ascii_whitespace();
    RangeConv {
        destination_start: numbers.next().unwrap().parse::<u64>().unwrap(),
        source_start: numbers.next().unwrap().parse::<u64>().unwrap(),
        size: numbers.next().unwrap().parse::<u64>().unwrap(),
    }
}

fn make_map(input: &str) -> Vec<RangeConv> {
    let mut lines = input.lines();
    lines.next();
    let mut result: Vec<RangeConv> = vec![];
    for line in lines {
        result.push(make_range(line));
    }
    result
}
