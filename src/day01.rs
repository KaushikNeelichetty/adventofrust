pub fn day01a(input: Vec<i32>) -> i32 {
    input.iter().map(|x| x / 3 - 2).fold(0, |acc, x| acc + x)
}

pub fn day01b(inputs: Vec<i32>) -> i32 {
    let mut final_fuel = 0;
    for input in inputs {
        let mut fuel: i32 = 0;
        let mut fuel_for_module: i32 = input;
        while fuel_for_module / 3 - 2 > 0 {
            fuel_for_module = fuel_for_module / 3 - 2;
            fuel = fuel + fuel_for_module;
        }
        final_fuel = final_fuel + fuel;
    }
    return final_fuel;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01a_test() {
        assert_eq!(day01a(vec![12, 14, 1969, 100756]), 34241);
    }

    #[test]
    fn day01b_test() {
        assert_eq!(day01b(vec![14, 1969, 100756]), 51314);
    }
}
