pub fn add(left: u64, right: u64) -> u64 {
    if left == 0 && right == 0 {
        panic!("Can't add 0 and 0");
    }
    left + right
}

fn private_add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn should_fail() {
        let result = add(2, 2);
        assert_eq!(result, 3, "This test is intended to fail!");
    }

    #[test]
    #[should_panic(expected="Can't add 0 and 0")]
    fn should_panic() {
        add(0, 0);
    }

    #[test]
    fn private_add_works() {
        let result = private_add(2, 2);
        assert_eq!(result, 4);
    }
}
