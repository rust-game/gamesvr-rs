//模块方法
pub trait Action {
    //before
    fn before();
    //do
    fn action();
    //after
    fn after();
}
