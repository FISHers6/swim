use route_recognizer::Params;
use std::sync::Arc;
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

    pub fn method(&self) -> http::method::Method {
        todo!()
    }

    pub fn url(&self) -> String {
        todo!()
    }
}
