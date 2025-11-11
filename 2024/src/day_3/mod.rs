use std::fs;

pub fn run() -> Result<(), ()> {
    let data = load_string_from_file();
    let part1 = calculate_muls_part1(&data);
    let part2 = calculate_muls_part2(&data);

    println!("Part 1 result: {part1}");
    println!("Part 2 result: {part2}");
    Ok(())
}

fn load_string_from_file() -> String {
    fs::read_to_string("./inputs/day_3.txt").expect("failed to load file")
}

/// Calculate muls from input part 1
///
/// This could be resolved rather easily with regex, but I wanted to implement it directly.
fn calculate_muls_part1(input: &str) -> u32 {
    let mut result: u32 = 0;
    let mut chars = input.chars();

    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    'outer: while let Some(c) = chars.next() {
        if c == 'm'
            && chars.next() == Some('u')
            && chars.next() == Some('l')
            && chars.next() == Some('(')
        {
            let mut first_number = String::new();
            for _ in 0..3 {
                if let Some(potential_digit) = chars.next() {
                    if digits.contains(&potential_digit) {
                        first_number += &potential_digit.to_string();
                    } else if potential_digit == ',' {
                        break;
                    } else {
                        continue 'outer;
                    }
                } else {
                    continue 'outer;
                }
            }

            // break if next character after a 3 digit number is not an `,`
            if first_number.len() == 3 && chars.next() != Some(',') {
                continue;
            }

            let mut second_number = String::new();
            for _ in 0..3 {
                if let Some(potential_digit) = chars.next() {
                    if digits.contains(&potential_digit) {
                        second_number += &potential_digit.to_string();
                    } else if potential_digit == ')' {
                        break;
                    } else {
                        continue 'outer;
                    }
                } else {
                    continue 'outer;
                }
            }

            // make sure we get a final `)` after 3 digits
            if second_number.len() == 3 && chars.next() != Some(')') {
                continue;
            }

            let parsed_num_0: u32 = first_number.parse().unwrap();
            let parsed_num_1: u32 = second_number.parse().unwrap();

            result += parsed_num_0 * parsed_num_1;
        }
    }

    result
}

/// Calculate muls from input part 2
///
/// This would also be considerably easier with regular expressions
fn calculate_muls_part2(input: &str) -> u32 {
    let mut result: u32 = 0;
    let mut chars = input.chars();
    let mut active = true;

    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    'outer: while let Some(c) = chars.next() {
        if active {
            // check if we should deactivate
            if c == 'd'
                && chars.next() == Some('o')
                && chars.next() == Some('n')
                && chars.next() == Some('\'')
                && chars.next() == Some('t')
                && chars.next() == Some('(')
                && chars.next() == Some(')')
            {
                active = false;
                continue;
            }

            // check if valid mul
            if c == 'm'
                && chars.next() == Some('u')
                && chars.next() == Some('l')
                && chars.next() == Some('(')
            {
                let mut first_number = String::new();
                for _ in 0..3 {
                    if let Some(potential_digit) = chars.next() {
                        if digits.contains(&potential_digit) {
                            first_number += &potential_digit.to_string();
                        } else if potential_digit == ',' {
                            break;
                        } else {
                            continue 'outer;
                        }
                    } else {
                        continue 'outer;
                    }
                }

                // break if next character after a 3 digit number is not an `,`
                if first_number.len() == 3 && chars.next() != Some(',') {
                    continue;
                }

                let mut second_number = String::new();
                for _ in 0..3 {
                    if let Some(potential_digit) = chars.next() {
                        if digits.contains(&potential_digit) {
                            second_number += &potential_digit.to_string();
                        } else if potential_digit == ')' {
                            break;
                        } else {
                            continue 'outer;
                        }
                    } else {
                        continue 'outer;
                    }
                }

                // make sure we get a final `)` after 3 digits
                if second_number.len() == 3 && chars.next() != Some(')') {
                    continue;
                }

                let parsed_num_0: u32 = first_number.parse().unwrap();
                let parsed_num_1: u32 = second_number.parse().unwrap();

                result += parsed_num_0 * parsed_num_1;
            }
        } else {
            // check if we should activate
            if c == 'd'
                && chars.next() == Some('o')
                && chars.next() == Some('(')
                && chars.next() == Some(')')
            {
                active = true;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_works() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(calculate_muls_part1(input), 161);
    }

    #[test]
    fn part2_example_works() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(calculate_muls_part2(input), 48);
    }

    #[test]
    fn three_digit_numbers_work() {
        let input = "xmul(200,4)%&mul[3,7]";
        assert_eq!(calculate_muls_part1(input), 800);
        let input = "xmul(21,400)%&mul[3,7]";
        assert_eq!(calculate_muls_part1(input), 8400);
        let input = "[12]asdxmul(784,456)";
        assert_eq!(calculate_muls_part1(input), 357504);
    }
}
