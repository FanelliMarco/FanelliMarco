pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let sum: u64 = num_string
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(num_string.len() as u32) as u64)
        .sum();

    sum == num as u64
}
