use route_recognizer::Params;
pub struct Request<State> {
    request: hyper::Request<hyper::body::Body>,
    state: State,
    params: Params,
}

impl<State> Request<State> {
    pub fn from_http(
        request: hyper::Request<hyper::body::Body>,
        state: State,
        params: Params,
    ) -> Self {
        Self {
            request,
            state,
            params,
        }
    }

    pub fn method(&self) -> &http::method::Method {
        self.request.method()
    }

    pub fn url(&self) -> String {
        self.request.uri().path().to_owned()
    }

    pub fn params(&self) -> &Params {
        &self.params
    }
}
