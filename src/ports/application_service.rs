trait ApplicationService<Req> {
    type Response;
    type Error;

    fn execute(&self, request: Req) -> Result<Self::Response, Self::Error>;
}
