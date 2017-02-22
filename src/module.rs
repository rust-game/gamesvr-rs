//这只定义模块
pub trait Module {

    //模块加载前
    fn before();

    //加载模块
    fn module();

   //模块加载后
    fn after();
}
