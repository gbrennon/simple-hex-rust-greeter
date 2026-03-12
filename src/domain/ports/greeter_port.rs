use std::error::Error;

use crate::domain::requests::greeter_request::GreeterRequest as Req;
use crate::domain::responses::greeter_response::GreeterResponse as Res;

pub trait GreeterPort {
    fn execute(&self, request: Req) -> Result<Res, impl Error>;
}
