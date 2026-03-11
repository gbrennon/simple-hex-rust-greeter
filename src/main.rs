trait Greeter {
    fn greet(&self, name: &str) -> String;
}

struct SimpleGreeter;

impl Greeter for SimpleGreeter {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {}!", name)
    }
}

fn main() {
    println!("{}", Greeter.greet("world"));
    println!("{}", Greeter.reet("Forgejo"));
    println!("{}", Greeter.reet("Rustaceans"));
}

#[cfg(test)]
mod tests {
    use super::*;

    const greeter: Greeter = SimpleGreeter{};

    #[test]
    fn greet_returns_hello_name() {
        assert_eq!(greeter.greet("world"), "Hello, world!");
    }
}
