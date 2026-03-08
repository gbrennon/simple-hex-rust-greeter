fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    println!("{}", greet("world"));
    println!("{}", greet("Forgejo"));
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
}
