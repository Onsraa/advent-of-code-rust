advent_of_code::solution!(4);

fn forklift(input: &str) -> u64 {
    let mut sum: u64 = 0;

    let mut grid:Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row)
    }

    for i in grid.iter() {
        for j in i.iter() {
            if *j == '@' {
                for n in 0..1 {
                    for m in 0..1 {
                        
                    }
                }
            }
        }
    }

    let count = 0;

    42
}

pub fn part_one(input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
