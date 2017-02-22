//! route 支持 map 一般游戏都是前后端固定的  支持自定义吧 暂时
pub trait Route {
    fn set();
    fn get();
}

//简单路由 默认的
struct SimpleRoute{

}

//实现简单的路由
impl Route for SimpleRoute{
    fn set(){}
    fn get(){

    }
}