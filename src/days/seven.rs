pub fn run(input: String) {
    let lines = input.as_str().lines();
    let mut hands: Vec<Hand> = vec![];
    for line in lines {
        hands.push(get_hand(line))
    }

    hands.sort();
    let mut total: u64 = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += hand.bid as u64 * (i as u64 + 1);
    }
    println!("{}", total);
}

pub fn runtwo(input: String) {
    let lines = input.as_str().lines();
    let mut hands: Vec<Hand> = vec![];
    for line in lines {
        hands.push(get_weird_hand(line))
    }

    hands.sort();
    let mut total: u64 = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += hand.bid as u64 * (i as u64 + 1);
    }
    println!("{}", total);
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    carts: [u8; 5],
    bid: u16,
    strength: u8,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.strength == other.strength {
            for (i, cart) in self.carts.iter().enumerate() {
                if cart == &other.carts[i] {
                    continue;
                }
                return cart.cmp(&other.carts[i]);
            }
        }
        self.strength.cmp(&other.strength)
    }
}

fn get_hand(input: &str) -> Hand {
    let mut blocks = input.split_ascii_whitespace();

    let mut carts: [u8; 5] = [0; 5];
    for (i, char) in blocks.next().unwrap().chars().enumerate() {
        carts[i] = match char.to_digit(10) {
            Some(num) => num as u8,
            None => match char {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Not a cart"),
            },
        };
    }
    let bid = blocks.next().unwrap().parse::<u16>().unwrap();
    let mut sorted_carts = carts;
    sorted_carts.sort();
    let mut last_num = 0;

    let mut length = 1u8;
    let mut extra_length = 1u8;
    for num in sorted_carts {
        if num != last_num {
            if length > 1 {
                if extra_length == 2 {
                    continue;
                }
                extra_length = length;
                length = 1u8;
            }
            last_num = num;
            continue;
        }
        length += 1;
    }
    if extra_length > length {
        std::mem::swap(&mut length, &mut extra_length);
    }
    let strength = match length {
        1 => 1,
        2 => {
            if extra_length == 2 {
                3
            } else {
                2
            }
        }
        3 => {
            if extra_length == 2 {
                5
            } else {
                4
            }
        }
        4 => 6,
        5 => 7,
        _ => panic!("AAAAAAAAAAAAAh"),
    };

    Hand {
        carts,
        bid,
        strength,
    }
}

fn get_weird_hand(input: &str) -> Hand {
    let mut blocks = input.split_ascii_whitespace();

    let mut carts: [u8; 5] = [0; 5];
    for (i, char) in blocks.next().unwrap().chars().enumerate() {
        carts[i] = match char.to_digit(10) {
            Some(num) => num as u8,
            None => match char {
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Not a cart"),
            },
        };
    }
    let bid = blocks.next().unwrap().parse::<u16>().unwrap();
    let mut sorted_carts = carts;
    sorted_carts.sort();
    let mut last_num = 0;

    let mut length = 1u8;
    let mut extra_length = 1u8;
    let mut extra_score = 0u8;
    for num in sorted_carts {
        if num == 1 {
            extra_score += 1;
            last_num = num;
            continue;
        }
        if num != last_num {
            if length > 1 {
                if extra_length == 2 {
                    continue;
                }
                extra_length = length;
                length = 1u8;
            }
            last_num = num;
            continue;
        }
        length += 1;
    }
    if extra_length > length {
        std::mem::swap(&mut length, &mut extra_length);
    }
    if extra_score == 5 {
        length = 5;
    } else {
        length += extra_score;
    }

    let strength = match length {
        1 => 1,
        2 => {
            if extra_length == 2 {
                3
            } else {
                2
            }
        }
        3 => {
            if extra_length == 2 {
                5
            } else {
                4
            }
        }
        4 => 6,
        5 => 7,
        _ => panic!("AAAAAAAAAAAAAh"),
    };

    Hand {
        carts,
        bid,
        strength,
    }
}
