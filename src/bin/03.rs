use std::collections::VecDeque;

advent_of_code::solution!(3);

fn find_largest_joltage(s: &str) -> u64 {
    let joltages: Vec<u64> = s
        .chars()
        .filter_map(|v| v.to_digit(10))
        .map(|d| d as u64)
        .collect();

    let len = joltages.len();

    let mut tens_index = 0;
    let mut unit_index;

    for i in 1..len - 1 {
        if joltages[i] > joltages[tens_index] {
            tens_index = i;
        }
    }

    unit_index = tens_index + 1;

    for i in unit_index..len {
        if joltages[i] > joltages[unit_index] {
            unit_index = i
        }
    }

    joltages[tens_index] * 10 + joltages[unit_index]
}

fn find_largest_joltage_by_size(s: &str, k: usize) -> u64 {
    let digits: Vec<u64> = s
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u64)
        .collect();

    let n = digits.len();

    let mut result_digits = Vec::with_capacity(k);
    let mut last_index = 0;

    for i in 0..k {
        let remaining_needed = k - 1 - i;
        let search_limit = n - remaining_needed;
        let mut max_digit = 0;
        let mut max_idx = last_index;

        for idx in last_index..search_limit {
            if digits[idx] > max_digit {
                max_digit = digits[idx];
                max_idx = idx;
            }
            if max_digit == 9 {
                break;
            }
        }

        result_digits.push(max_digit);
        last_index = max_idx + 1;
    }

    let mut final_number: u64 = 0;
    for &digit in &result_digits {
        final_number = final_number * 10 + digit;
    }

    final_number
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;

    for line in input.lines() {
        sum += find_largest_joltage(line)
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;

    for line in input.lines() {
        let largest = find_largest_joltage_by_size(line, 12);
        sum += largest
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
