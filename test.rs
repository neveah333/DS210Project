#[cfg(test)]
// test file used for testing the statistical analysis of the program
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let mut degrees = HashMap::new();
        degrees.insert("A".to_string(), 2);
        degrees.insert("B".to_string(), 3);
        degrees.insert("C".to_string(), 4);

        let mean_val = mean(&degrees);
        assert_eq!(mean_val, 3.0);
    }

    #[test]
    fn test_variance() {
        let mut degrees = HashMap::new();
        degrees.insert("A".to_string(), 2);
        degrees.insert("B".to_string(), 4);
        degrees.insert("C".to_string(), 4);

        let mean_val = mean(&degrees);
        let variance_val = variance(&degrees, mean_val);
        assert_eq!(variance_val, 1.0);
    }

    #[test]
    fn test_standard_deviation() {
        let mut degrees = HashMap::new();
        degrees.insert("A".to_string(), 1);
        degrees.insert("B".to_string(), 2);
        degrees.insert("C".to_string(), 3);

        let mean_val = mean(&degrees);
        let std_deviation = standard_deviation(&degrees, mean_val);
        assert!((std_deviation - 0.816496580927726).abs() < 1e-6);
    }

    #[test]
    fn test_mode() {
        let mut degrees = HashMap::new();
        degrees.insert("A".to_string(), 1);
        degrees.insert("B".to_string(), 2);
        degrees.insert("C".to_string(), 2);

        let mode_val = mode(&degrees);
        assert_eq!(mode_val, vec![2]);
    }
}