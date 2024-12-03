// Problem 1: Implement a function that adds two numbers and returns the result.
fn add(a: i32, b: i32) -> i32 {
    // Your code here
    unimplemented!()
}

// Problem 2: Implement a function that calculates the factorial of a number.
// The factorial of n (n!) is the product of all positive integers less than or equal to n.
// For example, factorial(5) should return 120.
fn factorial(n: u32) -> u32 {
    // Your code here
    unimplemented!()
}

// Problem 3: Implement a function that reverses a given string.
// For example, reverse_string("rust") should return "tsur".
fn reverse_string(s: &str) -> String {
    // Your code here
    unimplemented!()
}

// Problem 4: Implement a function that finds the maximum element in a vector of integers.
// The function should return None if the vector is empty.
fn max_in_vector(v: &Vec<i32>) -> Option<i32> {
    // Your code here
    unimplemented!()
}

fn main() {
    // You can test your functions here by calling them and printing the results
    println!("I am working")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(10, 5), 15);
        assert_eq!(add(-3, 3), 0);
        assert_eq!(add(-2, -2), -4);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(6), 720);
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("rust"), "tsur");
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string(""), "");
    }

    #[test]
    fn test_max_in_vector() {
        assert_eq!(max_in_vector(&vec![1, 3, 2, 5, 4]), Some(5));
        assert_eq!(max_in_vector(&vec![-1, -3, -2, -5, -4]), Some(-1));
        assert_eq!(max_in_vector(&vec![]), None);
    }
}
