pub trait Middleware{
    fn handle(context:Context,next:fn next() -> Middleware);
    fn next() -> Middleware;
}