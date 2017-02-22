pub trait Transfer{
    //读 read
    fn read();
    //写 write
    fn write();

    //open
    fn open();
    //close

    fn close_before();
    fn close();
    fn close_after();
}