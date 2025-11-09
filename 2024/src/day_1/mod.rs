use std::fs;

pub fn run() {
    load_file();
}

fn load_file() {
    let data = fs::read_to_string("./src/day_1/input.txt").expect("failed to load file");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in data.split('\n') {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // skip on empty lines
        if parts.len() == 0 {
            continue;
        }

        if parts.len() != 2 {
            println!("Invalid lenght!!!");
            continue;
        }

        left.push(parts[0].parse().expect("Failed to parse left"));
        right.push(parts[1].parse().expect("Failed to parse right"));
    }

    left.sort();
    right.sort();

    let mut distance: u32 = 0;
    for (i, l) in left.iter().enumerate() {
        distance += l.abs_diff(right[i]);
    }

    println!("The total distance is: {}", distance);
}
