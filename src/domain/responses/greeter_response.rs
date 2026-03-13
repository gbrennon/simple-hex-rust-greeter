#[derive(Debug, PartialEq)]
pub struct GreeterResponse {
    message: String,
}

impl GreeterResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, PartialEq)]
pub enum GreeterError {
    EmptyNotAllowed,
    NumberNotAllowed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_when_message_then_returns_greeter_response() {
        let message = "Hello, world!".to_string();

        let greeter_response = GreeterResponse::new(message.clone());

        let expected_response = GreeterResponse {
            message: message.clone(),
        };
        assert_eq!(greeter_response, expected_response);
    }

    #[test]
    fn new_when_empty_then_returns_greeter_response() {
        let message = "".to_string();

        let greeter_response = GreeterResponse::new(message.clone());

        let expected_response = GreeterResponse {
            message: message.clone(),
        };
        assert_eq!(greeter_response, expected_response);
    }

    #[test]
    fn new_when_long_message_then_returns_greeter_response() {
        let message = "a".repeat(100);

        let greeter_response = GreeterResponse::new(message.clone());

        assert_eq!(greeter_response.message, message);
    }

    #[test]
    fn message_returns_parameter_name() {
        let message = "Hello, world!".to_string();

        let greeter_response = GreeterResponse{
            message: message.clone(),
        };

        assert_eq!(greeter_response.message(), message);
    }
}
