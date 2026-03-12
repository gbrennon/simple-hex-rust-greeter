#[derive(Debug)]
pub struct GreeterRequest {
    name: String,
}

impl GreeterRequest {
    fn new(name: String) -> Self {
        Self { name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_when_name_then_returns_greet_request() {
        let name = "world".to_string();
        let greet_request = GreeterRequest::new(name.clone());

        assert_eq!(greet_request.name, name);
    }

    #[test]
    fn new_when_empty_then_returns_greet_request() {
        let name = "".to_string();
        let greet_request = GreeterRequest::new(name.clone());

        assert_eq!(greet_request.name, name);
    }

    #[test]
    fn new_when_long_name_then_returns_greet_request() {
        let name = "a".repeat(100);
        let greet_request = GreeterRequest::new(name.clone());

        assert_eq!(greet_request.name, name);
    }
}
