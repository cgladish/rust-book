pub fn add(left: u64, right: u64) -> u64 {
    if left == 0 && right == 0 {
        panic!("Can't add 0 and 0");
    }
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn should_fail() {
        let result = add(2, 2);
        assert_eq!(result, 3, "This test is intended to fail!");
    }

    #[test]
    #[should_panic(expected="Can't add 0 and 0")]
    fn should_panic() {
        add(0, 0);
    }
}
