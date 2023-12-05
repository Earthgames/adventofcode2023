use std::cmp::max;

pub fn run(input: String) {
    let lines = input.as_str().lines();
    let mut total: usize = 0;

    for (i, line) in lines.enumerate() {
        let sets = line.split(&[';']);
        let mut broke = false;

        for set in sets {
            let parts = set.split(&[',', ':']);
            let mut total_red: u32 = 0;
            let mut total_green: u32 = 0;
            let mut total_blue: u32 = 0;

            for part in parts {
                if part.contains("red") {
                    total_red += parse_int(part).unwrap();
                } else if part.contains("green") {
                    total_green += parse_int(part).unwrap();
                } else if part.contains("blue") {
                    total_blue += parse_int(part).unwrap();
                }
            }
            if total_red > 12 || total_green > 13 || total_blue > 14 {
                broke = true;
                break;
            }
        }

        if !broke {
            total += i + 1;
        }
    }
    println!("{}", total);
}

pub fn runtwo(input: String) {
    let lines = input.as_str().lines();
    let mut total: u32 = 0;

    for line in lines {
        let sets = line.split(&[';']);
        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;

        for set in sets {
            let parts = set.split(&[',', ':']);
            let mut total_red: u32 = 0;
            let mut total_green: u32 = 0;
            let mut total_blue: u32 = 0;

            for part in parts {
                if part.contains("red") {
                    total_red += parse_int(part).unwrap();
                } else if part.contains("green") {
                    total_green += parse_int(part).unwrap();
                } else if part.contains("blue") {
                    total_blue += parse_int(part).unwrap();
                }
            }

            max_red = max(total_red, max_red);
            max_green = max(total_green, max_green);
            max_blue = max(total_blue, max_blue);
        }

        total += max_red * max_blue * max_green;
    }
    println!("{}", total);
}

// I stole this shit(it's good stuff)
fn parse_int(input: &str) -> Option<u32> {
    input
        .chars()
        .skip_while(|ch| !ch.is_ascii_digit())
        .take_while(|ch| ch.is_ascii_digit())
        .fold(None, |acc, ch| {
            ch.to_digit(10).map(|b| acc.unwrap_or(0) * 10 + b)
        })
}
