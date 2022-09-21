pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

mod tests {
    #[test]
    fn test_sum() {
        let expected = 4;
        let result = super::sum(2, 2);
        assert_eq!(expected, result);
    }
    #[test]
    fn test_sum_err() {
        let expected = 5;
        let result = super::sum(2, 2);
        assert_ne!(expected, result);
    }
}
