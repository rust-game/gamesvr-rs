use response::Response;
use request::Request;
use context::Context;
use std::result::Result;

pub trait Middleware<Data>: 'static {
    fn handle(&self, req: &Request<Context<Data>>)  {
    }
}
