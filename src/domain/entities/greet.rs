pub struct Greeter {
    pub name: String,
}

impl Greeter {
    fn greet(&self) -> String {
        format!("Hey there, I'm {}!", self.name)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_when_world_then_returns_hello_world() {
        let greeter = Greeter {
            name: "world".to_string(),
        };

        assert_eq!(greeter.greet(), "Hey there, I'm world!");
    }

    #[test]
    fn greet_when_forgejo_then_returns_hello_forgejo() {
        let greeter = Greeter {
            name: "forgejo".to_string(),
        };

        assert_eq!(greeter.greet(), "Hey there, I'm forgejo!");
    }

    #[test]
    fn greet_when_empty_then_returns_hello() {
        let greeter = Greeter {
            name: "".to_string(),
        };

        assert_eq!(greeter.greet(), "Hey there, I'm !");
    }
}
