trait Greeter {
    fn greet(&self, name: &str) -> String;
}

struct SimpleGreeter;

impl Greeter for SimpleGreeter {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {}!", name)
    }
}

struct GreeterApplicationService {
    greeter: Box<dyn Greeter>,
}

impl GreeterApplicationService {
    fn new(greeter: Box<dyn Greeter>) -> Self {
        Self { greeter }
    }

    fn execute(&self, name: &str) -> String {
        self.greeter.greet(name)
    }
}

fn main() {
    let greeter = SimpleGreeter{};
    println!("{}", greeter.greet("world"));
    println!("{}", greeter.greet("Forgejo"));
    println!("{}", greeter.greet("Rustaceans"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_service_when_world_then_returns_hello_world() {
        let greeter = SimpleGreeter{};

        let service = GreeterApplicationService::new(Box::new(greeter));

        assert_eq!(service.execute("world"), "Hello, world!");
    }

    #[test]
    fn greet_service_when_forgejo_then_returns_hello_forgejo() {
        let greeter = SimpleGreeter{};

        let service = GreeterApplicationService::new(Box::new(greeter));

        assert_eq!(service.execute("Forgejo"), "Hello, Forgejo!");
    }

    #[test]
    fn greet_service_when_rustaceans_then_returns_hello_rustaceans() {
        let greeter = SimpleGreeter{};

        let service = GreeterApplicationService::new(Box::new(greeter));

        assert_eq!(service.execute("Rustaceans"), "Hello, Rustaceans!");
    }
}
