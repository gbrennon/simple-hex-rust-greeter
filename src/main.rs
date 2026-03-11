fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn sum_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", greet("world"));
    println!("{}", greet("Forgejo"));
    println!("{}", greet("Rustaceans"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_returns_hello_name() {
        assert_eq!(greet("world"), "Hello, world!");
    }

    #[test]
    fn greet_uses_provided_name() {
        assert_eq!(greet("Forgejo"), "Hello, Forgejo!");
    }

    #[test]
    fn greet_uses_provided_rustoceans() {
        assert_eq!(greet("Forgejo"), "Hello, Forgejo!");
    }

    #[test]
    fn sum_numbers_returns_correct_sum() {
        assert_eq!(sum_numbers(2, 3), 5);
    }

    #[test]
    fn sum_numbers_with_negative_values() {
        assert_eq!(sum_numbers(-2, -3), -5);
    }

    #[test]
    fn sum_numbers_with_mixed_values() {
        assert_eq!(sum_numbers(-2, 3), 1);
    }
}
