#[cfg(test)]
mod mul_div {
    use jd_rust::{operation, MathOperation};

    #[test]
    fn multiplication() {
        let result = operation(MathOperation {
            operator: "*".to_string(),
            left: 2,
            right: 2,
        });
        assert_eq!(result, 4);
    }

    #[test]
    fn division() {
        let result = operation(MathOperation {
            operator: "/".to_string(),
            left: 6,
            right: 2,
        });
        assert_eq!(result, 3);
    }
}
