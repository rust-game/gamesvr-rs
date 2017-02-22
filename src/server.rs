
pub trait Handle {
    fn handle(context:Context);
}


pub struct   Server {
    middleware Middleware
    handlers   []Handle
}

//服务器跑起来的方法
impl Server {
    //运行服务器
    fn run(){
    }
    //挂载中间件
    fn use_middleware(){

    }
}