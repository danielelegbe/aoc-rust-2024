advent_of_code::solution!(1);

pub fn create_and_sort_lists(input: &str) -> Option<(Vec<u32>, Vec<u32>)> {
    let mut list_one = vec![];
    let mut list_two = vec![];

    for line in input.lines() {
        let mut nums = line.split_whitespace();

        let num_one: u32 = nums.next()?.parse().unwrap();
        let num_two: u32 = nums.next()?.parse().unwrap();

        list_one.push(num_one);
        list_two.push(num_two);
    }

    list_one.sort();
    list_two.sort();

    Some((list_one, list_two))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    let (list_one, list_two) = create_and_sort_lists(input)?;

    for i in 0..list_one.len() {
        let num_one = list_one[i];
        let num_two = list_two[i];

        let difference = num_one.abs_diff(num_two);

        sum += difference;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    let (list_one, list_two) = create_and_sort_lists(input)?;

    for left_num in list_one {
        let mut num_of_occurences = 0;

        for right_num in &list_two {
            if left_num == *right_num {
                num_of_occurences += 1
            }
        }

        sum += left_num * num_of_occurences;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
