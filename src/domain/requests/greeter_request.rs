#[derive(Debug, PartialEq)]
pub struct GreeterRequest {
    name: String,
}

impl GreeterRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_when_name_then_returns_greet_request() {
        let name = "world".to_string();

        let greeter_request = GreeterRequest::new(name.clone());

        let expected_request = GreeterRequest {
            name: "world".to_string(),
        };
        assert_eq!(greeter_request, expected_request);
    }

    #[test]
    fn new_when_empty_then_returns_greet_request() {
        let name = "".to_string();

        let greeter_request = GreeterRequest::new(name.clone());

        let expected_request = GreeterRequest {
            name: "".to_string(),
        };
        assert_eq!(greeter_request, expected_request);
    }

    #[test]
    fn new_when_long_name_then_returns_greet_request() {
        let name = "a".repeat(100);
        let greeter_request = GreeterRequest::new(name.clone());

        assert_eq!(greeter_request.name(), name);
    }
}
