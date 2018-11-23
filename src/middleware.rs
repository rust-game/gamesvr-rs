use response::Response;
use request::Request;
use std::result::Result;

pub trait Middleware<Data>: 'static {
    /// Method is called when request is ready. It may return
    /// future, which should resolve before next middleware get called.
    fn start(&self, req: &Request<Data>)  {

    }

    /// Method is called when handler returns response,
    /// but before sending http message to peer.
    fn response(&self, req: &Request<Data>, resp: Response)  {

    }

    /// Method is called after body stream get sent to peer.
    fn finish(&self, req: &Request<S>, resp: &Response) -> Finished {
        Finished::Done
    }
}
