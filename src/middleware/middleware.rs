pub trait Middleware{
    fn next() -> Middleware;
}