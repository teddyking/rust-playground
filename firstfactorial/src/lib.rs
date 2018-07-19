pub fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }

    n * factorial(n - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod factorial {
        use super::*;

        #[test]
        fn with_zero() {
            let n = 0;
            assert_eq!(factorial(n), 1);
        }

        #[test]
        fn with_positive_integer() {
            let n = 1;
            assert_eq!(factorial(n), 1);

            let n = 4;
            assert_eq!(factorial(n), 24);

            let n = 8;
            assert_eq!(factorial(n), 40320);
        }
    }
}
