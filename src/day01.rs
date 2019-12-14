pub fn day01a(input: Vec<i32>) -> i32 {
    input.iter().map(|x| x / 3 - 2).fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01a_test() {
        assert_eq!(day01a(vec![12, 14, 1969, 100756]), 34241);
    }
}
