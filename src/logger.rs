use middleware::Middleware;
use request::Request;
use context::Context;

pub struct  Logger{

}

impl Logger{
    fn new() -> Self {
        Logger{}
    }
}

impl Middleware for Logger{
    fn handle(&self, req: &Request<Context<Data>>)  {
       println!(" Logger ");
    }
}
