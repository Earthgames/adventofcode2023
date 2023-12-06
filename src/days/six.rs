use std::u64;

pub fn run(input: String) {
    let mut lines = input.as_str().lines();

    let times = get_input_vec(lines.next().unwrap());
    let distances = get_input_vec(lines.next().unwrap());

    let mut races: Vec<Race> = vec![];
    for (i, _) in times.iter().enumerate() {
        races.push(Race {
            max_time: times[i],
            distance: distances[i],
        })
    }

    let mut result: u64 = 1;
    for race in races {
        let (mut min_time, max_time) = race.get_times(race.distance).unwrap();
        if race.get_distance(min_time).unwrap() == race.distance {
            min_time += 1;
        }
        if min_time == max_time {
            continue;
        }
        result *= max_time - min_time;
    }
    println!("{}", result);
}

struct Race {
    max_time: u64,
    distance: u64,
}

impl Race {
    fn get_times(&self, distance: u64) -> Option<(u64, u64)> {
        let discriminant: f64 =
            self.max_time as f64 * self.max_time as f64 + 4f64 * -1f64 * (distance as f64);
        if discriminant >= 0f64 {
            let result = (-1f64 * self.max_time as f64 + discriminant.sqrt()) / -2f64;
            let result2 = (-1f64 * self.max_time as f64 - discriminant.sqrt()) / -2f64;
            return Some((result.ceil() as u64, result2.ceil() as u64));
        }
        None
    }

    fn get_distance(&self, time: u64) -> Option<u64> {
        let result = -1 * (time * time) as i64 + (self.max_time * time) as i64;
        if result < 0 {
            return None;
        }
        Some(result as u64)
    }
}

fn get_input_vec(input: &str) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    for time in input
        .split(':')
        .next_back()
        .unwrap()
        .split_ascii_whitespace()
    {
        result.push(match time.parse::<u64>() {
            Ok(t) => t,
            Err(_) => continue,
        })
    }
    result
}
