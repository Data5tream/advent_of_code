use std::fs;

#[derive(Debug)]
struct FileData {
    left: Vec<i32>,
    right: Vec<i32>,
}

pub fn run() -> Result<(), ()> {
    let data = load_file();
    let distance = calculate_result(data);
    println!("The distance is {distance}");
    Ok(())
}

fn load_file() -> FileData {
    let data = fs::read_to_string("./inputs/day_1.txt").expect("failed to load file");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in data.split('\n') {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // skip on empty lines
        if parts.is_empty() {
            continue;
        }

        if parts.len() != 2 {
            println!("Invalid lenght!!!");
            continue;
        }

        left.push(parts[0].parse().expect("Failed to parse left"));
        right.push(parts[1].parse().expect("Failed to parse right"));
    }

    FileData { left, right }
}

fn calculate_result(data: FileData) -> u32 {
    let mut left = data.left;
    let mut right = data.right;

    left.sort();
    right.sort();

    let mut distance: u32 = 0;
    for (i, l) in left.iter().enumerate() {
        distance += l.abs_diff(right[i]);
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = FileData {
            left: vec![3, 4, 2, 1, 3, 3],
            right: vec![4, 3, 5, 3, 9, 3],
        };

        assert_eq!(calculate_result(input), 11);
    }
}
