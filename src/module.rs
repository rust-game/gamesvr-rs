//这只定义模块
pub trait Module {

    //模块加载前
    fn before();

    //加载模块
    fn add_action();

   //模块加载后
    fn after();
}


//模块栈
pub struct  ModuleStack {
    modules: Vec<Box<Module>>,
}


impl ModuleStack{
    fn add_action<A:Module>(&mut self,a:A){
        self.modules.push(a);
    }

    pub fn new()->ModuleStack{
        ModuleStack{
            modules:Vec::new()
        }
    }
}
