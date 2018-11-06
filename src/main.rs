pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_add_zero() {
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    pub fn test_add_one() {
        assert_eq!(add(1, 1), 2);
    }

    #[test]
    pub fn test_add_sum_zero() {
        assert_eq!(add(1, -1), 0);
    }

    #[test]
    pub fn test_add_positive() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    pub fn test_add_negative() {
        assert_eq!(add(-1, -1), -2);
    }
}
