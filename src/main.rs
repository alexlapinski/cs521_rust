use std::fmt::Debug;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let arr:[i32; 5] = [1, 2, 3, 4, 5];
    let arr2:[i32; 5] = [4, 2, 1, 5, 3];

    print_array("Input:", &arr2);
    print_array("Output:", &arr);
}

fn print_array<T: Debug>(label:&str, array:&[T]) {
    println!{"{:8} {:?}", label, array};
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
