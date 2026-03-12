pub struct Greeter {
    name: String,
}

impl Greeter {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn greet(&self) -> String {
        format!("Hey there, I'm {}!", self.name)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_when_name_then_returns_greeter() {
        let greeter = Greeter::new("world");

        assert_eq!(greeter.name(), "world");
    }

    #[test]
    fn greet_when_world_then_returns_hello_world() {
        let greeter = Greeter::new("world");

        assert_eq!(greeter.greet(), "Hey there, I'm world!");
    }

    #[test]
    fn greet_when_forgejo_then_returns_hello_forgejo() {
        let greeter = Greeter::new("forgejo");

        assert_eq!(greeter.greet(), "Hey there, I'm forgejo!");
    }

    #[test]
    fn greet_when_empty_then_returns_hello() {
        let greeter = Greeter::new("");

        assert_eq!(greeter.greet(), "Hey there, I'm !");
    }

    #[test]
    fn name_when_foo_then_return_foo() {
        let greeter = Greeter::new("Foo");

        assert_eq!(greeter.name(), "Foo");
    }
}
