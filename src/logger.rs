pub trait Logger {
    fn debug();
    fn info();
    fn warn();
    fn error();
    fn fatal();
    fn on();
    fn off();
    fn is_on();
}

//默认
pub struct SimpleLogger {

}

//实现
impl  Logger for SimpleLogger{
    fn debug(){}
    fn info(){}
    fn warn(){}
    fn error(){}
    fn fatal(){}
    fn on(){}
    fn off(){}
    fn is_on(){}
}