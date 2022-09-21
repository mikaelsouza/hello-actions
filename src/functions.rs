pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
mod tests {
    use super::sum;
    #[test]
    fn test_sum() {
        let expected = 4;
        let result = super::sum(2, 2);
        assert_eq!(expected, result);
    }
}
