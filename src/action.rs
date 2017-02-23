//模块方法
pub trait Action {
    //before
    fn before();
    //do
    fn action();
    //after
    fn after();
}

//方法栈
pub struct ActionStack{
    actions: Vec<Box<Action>>,
}

impl ActionStack{
    fn add_action<A:Action>(&mut self,a:A){
        self.actions.push(a);
    }

    pub fn new()->ActionStack{
        ActionStack{
            actions:Vec::new()
        }
    }
}
