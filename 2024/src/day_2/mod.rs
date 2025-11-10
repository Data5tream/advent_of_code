use std::{
    fs::File,
    io::{BufReader, prelude::*},
};

pub fn run() {
    let file = File::open("./src/day_2/input.txt").expect("failed to load file");
    let reader = BufReader::new(file);
    let mut safe_reports: i16 = 0;

    for report in reader.lines() {
        let entries: Vec<i32> = report
            .unwrap()
            .split_whitespace()
            .map(|a| a.parse().unwrap())
            .collect();

        // first try, might be safe
        if report_is_save(entries.clone()) {
            safe_reports += 1;
            continue;
        }

        let mut drop_index = 0;
        let entry_count = entries.len();

        // Check if safe if we remove one of the levels. This is quite slow, could probably be made
        // a lot faster.
        while drop_index < entry_count {
            let mut try_vec = entries.clone();
            try_vec.remove(drop_index);
            if report_is_save(try_vec) {
                safe_reports += 1;
                break;
            }

            drop_index += 1;
        }
    }

    println!("Safe reports: {safe_reports}");
}

fn report_is_save(entries: Vec<i32>) -> bool {
    let mut index = 0;
    let entry_count = entries.len() - 1;
    let mut is_increasing: Option<bool> = None;

    while index < entry_count {
        let a = entries[index];
        let b = entries[index + 1];

        let diff = a.abs_diff(b);
        if diff > 3
            || diff < 1
            || (is_increasing.is_some_and(|x| x) && a > b)
            || (is_increasing.is_some_and(|x| !x) && a < b)
        {
            return false;
        }

        if is_increasing.is_none() {
            is_increasing = Some(a < b);
        }

        index += 1;
    }

    return true;
}
