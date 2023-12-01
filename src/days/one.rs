pub fn run(input: String) {
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
    println!("{total}");
}

pub fn runtwo(input: String) {
    let lines = input.as_str().lines();
    let mut total: u32 = 0;
    for line in lines {
        let mut word: String = "".to_string();
        let mut local_total: Vec<char> = vec![];

        for char in line.chars() {
            if char.is_ascii_digit() {
                local_total.push(char);
                word = "".to_string();
            } else {
                word.push(char);
                if word.contains("one") {
                    local_total.push('1');
                    word.drain(0..(word.len() - 2));
                } else if word.contains("two") {
                    local_total.push('2');
                    word.drain(0..(word.len() - 2));
                } else if word.contains("three") {
                    local_total.push('3');
                    word.drain(0..(word.len() - 2));
                } else if word.contains("four") {
                    local_total.push('4');
                    word = "".to_string();
                } else if word.contains("five") {
                    local_total.push('5');
                    word.drain(0..(word.len() - 2));
                } else if word.contains("six") {
                    local_total.push('6');
                    word.drain(0..(word.len() - 2));
                } else if word.contains("seven") {
                    local_total.push('7');
                    word.drain(0..(word.len() - 2));
                } else if word.contains("eight") {
                    local_total.push('8');
                    word.drain(0..(word.len() - 2));
                } else if word.contains("nine") {
                    local_total.push('9');
                    word.drain(0..(word.len() - 2));
                }
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
    println!("{total}");
}
