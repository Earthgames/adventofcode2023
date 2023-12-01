use super::get_input;

pub fn run() {
    let input = get_input(1);
    let lines = input.as_str().lines();
    let mut total: u32 = 0;
    for line in lines {
        let mut local_total: Vec<char> = vec![];
        for char in line.chars() {
            if char.is_ascii_digit() {
                local_total.push(char);
            }
        }
        let local_total: String = format!(
            "{}{}",
            &local_total[0],
            match local_total.last() {
                Some(a) => a,
                None => &local_total[0],
            }
        );
        total += local_total.parse::<u32>().unwrap();
    }
    print!("{total}");
}
