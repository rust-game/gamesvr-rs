pub struct Response<Body>{
    body:Body
}

impl Response<Body>{
    fn new() ->Self{
        Response{
            body:Body
        }
    }
}
