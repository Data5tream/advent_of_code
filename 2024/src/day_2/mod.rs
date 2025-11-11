use std::{
    fs::File,
    io::{BufReader, prelude::*},
};

pub fn run() -> Result<(), ()> {
    let file = File::open("./inputs/day_2.txt").expect("failed to load file");
    let reader = BufReader::new(file);
    let mut safe_reports: i16 = 0;

    for report in reader.lines() {
        let entries: Vec<i32> = report
            .unwrap()
            .split_whitespace()
            .map(|a| a.parse().unwrap())
            .collect();

        if check_report_with_damper(entries) {
            safe_reports += 1;
        }
    }

    println!("Safe reports: {safe_reports}");
    Ok(())
}

fn check_report_with_damper(entries: Vec<i32>) -> bool {
    // first try, might be safe
    if report_is_save(entries.clone()) {
        return true;
    }

    let mut drop_index = 0;
    let entry_count = entries.len();

    // Check if safe if we remove one of the levels. This is quite slow, could probably be made
    // a lot faster.
    while drop_index < entry_count {
        let mut try_vec = entries.clone();
        try_vec.remove(drop_index);
        if report_is_save(try_vec) {
            return true;
        }

        drop_index += 1;
    }

    false
}

fn report_is_save(entries: Vec<i32>) -> bool {
    let mut index = 0;
    let entry_count = entries.len() - 1;
    let mut is_increasing: Option<bool> = None;

    while index < entry_count {
        let a = entries[index];
        let b = entries[index + 1];

        let diff = a.abs_diff(b);
        if !(1..=3).contains(&diff)
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

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_ascending_report_is_safe() {
        assert_eq!(report_is_save(vec![1, 3, 4, 5, 8, 11]), true);
    }

    #[test]
    fn simple_ascending_report_is_unsafe_because_of_gaps() {
        assert_eq!(report_is_save(vec![1, 3, 4, 5, 11]), false);
    }

    #[test]
    fn simple_descending_report_is_safe() {
        assert_eq!(report_is_save(vec![20, 18, 16, 15, 14]), true);
    }

    #[test]
    fn simple_descending_report_is_unsafe_because_of_gaps() {
        assert_eq!(report_is_save(vec![37, 36, 35, 10, 9, 8]), false);
    }

    #[test]
    fn simple_report_is_unsafe_because_of_direction_switch() {
        assert_eq!(report_is_save(vec![1, 3, 4, 5, 4, 1]), false);
    }

    #[test]
    fn simple_report_is_unsafe_because_of_duplicate_numbers() {
        assert_eq!(report_is_save(vec![1, 1, 3, 4, 5]), false);
    }

    #[test]
    fn damper_report_is_safe() {
        assert_eq!(check_report_with_damper(vec![1, 3, 4, 5, 8, 11]), true);
    }

    #[test]
    fn damper_report_is_safe_with_out_of_limits_at_start() {
        assert_eq!(check_report_with_damper(vec![1, 6, 7, 8, 11]), true);
    }

    #[test]
    fn damper_report_is_safe_with_direction_switch_at_start() {
        assert_eq!(check_report_with_damper(vec![5, 3, 4, 5, 8, 11]), true);
    }

    #[test]
    fn damper_report_is_safe_with_skipped_inbetween() {
        assert_eq!(check_report_with_damper(vec![1, 3, 7, 4, 6, 8]), true);
    }

    #[test]
    fn damper_report_is_safe_with_out_of_limits_at_end() {
        assert_eq!(check_report_with_damper(vec![1, 3, 4, 5, 8, 18]), true);
    }

    #[test]
    fn damper_report_is_safe_with_direction_switch_at_end() {
        assert_eq!(check_report_with_damper(vec![1, 3, 4, 5, 8, 6]), true);
    }

    #[test]
    fn damper_report_is_safe_with_double_number_at_start() {
        assert_eq!(check_report_with_damper(vec![1, 1, 3, 4, 5, 8, 11]), true);
    }

    #[test]
    fn damper_report_is_safe_with_double_number_inbetween() {
        assert_eq!(check_report_with_damper(vec![1, 3, 4, 4, 5, 8, 11]), true);
    }

    #[test]
    fn damper_report_is_safe_with_double_number_at_end() {
        assert_eq!(check_report_with_damper(vec![1, 3, 4, 5, 8, 11, 11]), true);
    }

    #[test]
    fn damper_report_is_unsafe_with_double_skip() {
        assert_eq!(
            check_report_with_damper(vec![1, 3, 4, 4, 4, 5, 8, 11]),
            false
        );
    }
}
