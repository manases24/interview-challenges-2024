// Even or Odd

// Create a function that takes an integer as an argument and returns "Even" for even numbers or "Odd" for odd numbers

// Link: https://www.codewars.com/kata/53da3dbb4a5168369a0000fe

fn even_or_odd(number: i32) -> &'static str {
    if number % 2 == 0 {
        return "Even"
    } else {
        return "Odd"
    }
}

#[cfg(test)]
mod sample_tests {
    use super::even_or_odd;
    
    fn do_test(number: i32, expected: &str) {
        let actual = even_or_odd(number);
        assert_eq!(actual, expected, "\nYour result (left) does not match the expected output (right) for the input {number:?}");
    }

    #[test]
    fn test_zero() {
        do_test(0, "Even");
    }

    #[test]
    fn test_positive_even() {
        do_test(2, "Even");
    }

    #[test]
    fn test_positive_odd() {
        do_test(1, "Odd");
    }

    #[test]
    fn test_negative_even() {
        do_test(-2, "Even");
    }

    #[test]
    fn test_negative_odd() {
        do_test(-1, "Odd");
    }
}