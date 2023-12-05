pub fn run(input: String) {
    let mut blocks  = input.split("\n\n");
    let mut seeds: Vec<u64> = vec![];

    for seed in blocks.next().unwrap().split(':').next_back().unwrap().split_ascii_whitespace(){
        seeds.push(match seed.parse::<u64>(){
            Ok(t) => t,
            Err(_) => continue,
        })
    }


    let mut maps: Vec<Vec<Range>> = vec![];
    for block in blocks{
        maps.push(make_map(block));
    }
    let mut final_num: Vec<u64> = vec![];
    for number in seeds{
        let mut current_num = number;
        for map in &maps{
            current_num = convert_with_map(map, current_num);
        }
        final_num.push(current_num);
    }
    final_num.sort();
    println!("{:?}", final_num[0]);
}

struct Range{
    source_start: u64,
    destination_start: u64,
    size: u64,
}

fn convert_with_map(map: &Vec<Range>, source_number: u64) -> u64
    {
        for range in map{
            match range_number(range, source_number) {
               Some(num) => return num,
               None => continue,
            }
        }
        source_number
    }

fn range_number(range: &Range, source_number: u64) -> Option<u64>{
    if source_number < range.source_start || source_number > range.source_start + range.size - 1{
        return None;
    }
    Some (source_number + range.destination_start - range.source_start )
}

fn make_range(input: &str) -> Range{
    let mut numbers = input.split_ascii_whitespace();
    Range {
        destination_start : numbers.next().unwrap().parse::<u64>().unwrap(),
        source_start : numbers.next().unwrap().parse::<u64>().unwrap(),
        size : numbers.next().unwrap().parse::<u64>().unwrap(),
    }
}

fn make_map(input: &str) -> Vec<Range>{
    let mut lines = input.lines();
    lines.next();
    let mut result : Vec<Range>= vec![];
    for line in lines{
        result.push(make_range(line));
    }
    result
}