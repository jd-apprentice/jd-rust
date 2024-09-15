#[cfg(test)]
mod add_sub {
    use jd_rust::{operation, MathOperation};

    #[test]
    fn addition() {
        let result = operation(MathOperation {
            operator: "+".to_string(),
            left: 2,
            right: 2,
        });
        assert_eq!(result, 4);
    }

    #[test]
    fn substraction() {
        let result = operation(MathOperation {
            operator: "-".to_string(),
            left: 4,
            right: 2,
        });
        assert_eq!(result, 2);
    }
}
