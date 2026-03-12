#[derive(Debug)]
pub struct GreeterResponse{
    message: String,
}

impl GreeterResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_when_message_then_returns_greet_response() {
        let message = "Hello, world!".to_string();

        let greet_response = GreeterResponse::new(message.clone());

        assert_eq!(greet_response.message, message);
    }

    #[test]
    fn new_when_empty_then_returns_greet_response() {
        let message = "".to_string();

        let greet_response = GreeterResponse::new(message.clone());

        assert_eq!(greet_response.message, message);
    }

    #[test]
    fn new_when_long_message_then_returns_greet_response() {
        let message = "a".repeat(100);

        let greet_response = GreeterResponse::new(message.clone());

        assert_eq!(greet_response.message, message);
    }
}
