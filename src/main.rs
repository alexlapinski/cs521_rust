use std::fmt::Debug;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5];
    let arr2: Vec<i32> = vec![4, 2, 1, 5, 3];

    print_vector("Input:", arr2);

    print_vector("Output:", arr);
}

fn print_vector<T: Debug>(label:&str, vector:Vec<T>) {
    println!{"{:8} {:?}", label, vector};
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
