use self::Context;

pub trait Middleware{
    fn invoke<T:Context>(context:T){
    }
}

//调用的一个栈
pub struct MiddlewareStack{
    handlers: Vec<Box<Middleware>>,
}

impl MiddlewareStack{
    pub  fn add<M:Middleware>(&mut self,m:M){
        self.handlers.push(m)
    }
    //调用这个栈
    pub fn invoke<T>(&self,context:T) where T:Context{

    }

    pub fn new()->MiddlewareStack{
        MiddlewareStack{
            handlers:Vec::new()
        }
    }
}
