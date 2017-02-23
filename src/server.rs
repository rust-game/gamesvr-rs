use {MiddlewareStack,Middleware,ModuleStack};

//服务器接口
//一个服务监听一个端口
//一个服务必须有且只有一个传输协议
//一个服务必须有且只有一个解包的协议
//一个服务有零个或者1个压缩协议
//一个服务必须有且只有一个路由
//TODO 关于加密协议 有点不知道放哪 --哪一步都可能要加密
pub trait   Server {
    //运行服务器
    //ip 地址 //默认监听0.0.0.0 port 必填 ..

    fn run(port:&'static str,ip:&'static str);

    //设置日志记录的
    fn use_logger<M:Middleware>(&mut self ,m:M,level:String);

    fn use_transfer<M:Middleware>(&mut self,m:M);

    fn use_compress<M:Middleware>(&mut self,m:M) ;

    fn use_serialize<M:Middleware>(&mut self,m:M);

    fn use_route<M:Middleware>(&mut self,m:M);

    //挂载中间件
    fn add_middleware<M:Middleware>(&mut self,m:M);
   //挂载模块
    fn add_module<M:ModuleStack>(M);

    fn new()->Server;

    //构建
    fn build()->MiddlewareStack{
    }

}





//服务器跑起来的方法
impl Server {
    //运行服务器
    fn run(){
    }
    //挂载中间件
    fn use_middleware<M>(&mut self,middleware:M) where M:Middleware{
        self.middleware.add()
    }
    fn new()->Server{

    }
}

//返回这个简单挂载好部分常用组件的
pub  struct  TcpServer {
}
//
pub  struct  UdpServer {
}

pub  struct  HttpServer {
}

pub  struct  WebSocketServer {
}