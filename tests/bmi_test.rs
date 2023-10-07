#[path = "../src/bmi.rs"]
mod bmi;

#[cfg(test)]
mod tests {
    use super::bmi::calculator;

    #[test]
    fn test_calculator() {
        let result = calculator(180, 65);
        assert_eq!(result, 20.1);
    }
}
