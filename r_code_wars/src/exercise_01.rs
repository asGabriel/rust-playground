/*
In this little assignment you are given a string of space separated numbers, and have to return the highest and lowest number.

Examples
high_and_low("1 2 3 4 5")  // return "5 1"
high_and_low("1 2 -3 4 5") // return "5 -3"
high_and_low("1 9 3 4 -5") // return "9 -5"
Notes
All numbers are valid Int32, no need to validate them.
There will always be at least one number in the input string.
Output string must be two numbers separated by a single space, and highest number is first.
*/

pub fn high_and_low(numbers: &str) -> String {
    let mut numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect();

    numbers.sort_unstable();

    format!("{} {}", numbers[numbers.len() - 1], numbers[0])
}

#[test]
fn example_test_1() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn example_test_2() {
    assert_eq!("3 1", high_and_low("1 2 3"));
}
