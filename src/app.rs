use middleware::Middleware;
pub struct App<Data> {
    data: Data,
    router: Router<Data>,
    middleware: Vec<Box<dyn Middleware<Data> + Send + Sync>>,
}
