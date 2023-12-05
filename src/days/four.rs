pub fn run(input: String) {
    let lines = input.as_str().lines();
    let mut total: u32 = 0;
    for line in lines {
        let (winning_numbers, owend_numbers) = get_numbers(line);
        let matching_numbers = get_matching_numbers(&winning_numbers, &owend_numbers);
        if !matching_numbers.is_empty() {
         total += 2_u32.pow(matching_numbers.len() as u32 -1)
        }
    }
    println!("{total}");
}

pub fn runtwo(input: String) {
    let lines = input.as_str().lines();
    let mut copies: Vec<u32> = vec![1; lines.clone().count()];
    for (i, line) in lines.enumerate() {
        let (winning_numbers, owend_numbers) = get_numbers(line);
        let matching_numbers = get_matching_numbers(&winning_numbers, &owend_numbers);
        for _ in 0..copies[i]{
        for (number, _) in matching_numbers.iter().enumerate() {
            copies[i+number + 1] += 1;
        }
    }
    }
    println!("{}", copies.iter().sum::<u32>() );
}

fn get_numbers(input: &str) -> (Vec<u32>, Vec<u32>){
        let mut winning_numbers :Vec<u32> = vec![];
        let mut owend_numbers :Vec<u32> = vec![];
        let mut numbers = input.split(':').next_back().unwrap().split('|');
        let winning_numbers_str = numbers.next().unwrap();
        let owend_numbers_str = numbers.next().unwrap();
        for number in winning_numbers_str.split_ascii_whitespace(){
            winning_numbers.push(number.parse::<u32>().unwrap())
        }
        for number in owend_numbers_str.split_ascii_whitespace(){
            owend_numbers.push(number.parse::<u32>().unwrap())
        }
        (winning_numbers, owend_numbers)
}

fn get_matching_numbers(vec_source: &Vec<u32>, vec_match: &Vec<u32>) -> Vec<u32>{
    let mut result: Vec<u32> = vec![];
    for number in vec_match{
        if vec_source.contains(number)
            {
                result.push(number.to_owned())
            }
    }
    result
}