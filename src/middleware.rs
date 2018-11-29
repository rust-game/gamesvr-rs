use response::Response;
use request::Request;
use std::result::Result;

pub trait Middleware<Data>: 'static {
    fn handle(&self, req: &Request<Data>)  {
    }
}
