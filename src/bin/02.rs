advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_counter = 0;

    'outer: for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let differences: Vec<i32> = nums
            .windows(2)
            .map(|pair| {
                let first = pair[0];
                let second = pair[1];

                second - first
            })
            .collect();

        for num in &differences {
            if num.abs() > 3 || num.abs() < 1 {
                continue 'outer;
            }
        }

        let all_positive = differences.iter().all(|num| *num > 0);

        let all_negative = differences.iter().all(|num| *num < 0);

        if all_positive == all_negative {
            continue 'outer;
        }

        safe_counter += 1
    }

    Some(safe_counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
