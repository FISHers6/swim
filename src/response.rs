use hyper::Body;

pub struct Response {
    pub response: hyper::Response<Body>,
}

impl From<Response> for hyper::Response<Body> {
    fn from(response: Response) -> Self {
        todo!()
    }
}
