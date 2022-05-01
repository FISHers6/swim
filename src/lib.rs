mod chain;
mod handler;
mod middleware;
mod request;
mod response;
mod router;

use crate::chain::Chain;
use crate::middleware::Middleware;
use crate::request::Request;
use crate::response::Response;
use crate::router::Router;
use futures_util::{future, TryFutureExt};
use hyper::service::Service;
use hyper::Error;
use hyper::Server;
use route_recognizer::Match;
use std::convert::Infallible;
use std::future::Future;
use std::net::{SocketAddr, ToSocketAddrs};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

pub type Result<T = Response> = std::result::Result<T, Error>;

pub struct App<State> {
    state: State,
    router: Arc<Router<State>>,
    middlewares: Arc<Vec<Arc<dyn Middleware<State>>>>,
}

impl App<()> {
    fn new() -> Self {
        Self::build(())
    }
}

impl<State> App<State>
where
    State: Clone + Send + Sync + 'static,
{
    pub fn build(state: State) -> Self {
        App {
            state,
            router: Arc::new(Router::new()),
            middlewares: Arc::new(vec![]),
        }
    }

    pub fn with<M: Middleware<State>>(&mut self, middleware: M) -> &mut Self {
        let middlewares = Arc::get_mut(&mut self.middlewares).expect("Can not find middlewares");
        middlewares.push(Arc::new(middleware));
        self
    }

    pub async fn swim<A>(self, addr: A)
    where
        A: ToSocketAddrs,
    {
        let addr: SocketAddr = addr.to_socket_addrs().unwrap().next().unwrap();
        let server = Server::bind(&addr)
            .serve(MakeService(self))
            .map_err(|e| println!("server error: {}", e));
        let _ = server.await;
    }
}

impl<State: Clone> Clone for App<State> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            router: self.router.clone(),
            middlewares: self.middlewares.clone(),
        }
    }
}

impl<State: Clone + Send + Sync + 'static> Service<hyper::Request<hyper::body::Body>>
    for App<State>
{
    type Response = hyper::Response<hyper::body::Body>;
    type Error = Error;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, req: hyper::Request<hyper::body::Body>) -> Self::Future {
        let Self {
            state,
            router,
            middlewares,
        } = self.clone();
        let future = async move {
            // 取出路由
            let method = req.method().to_owned();
            let Match { handler, params } = router.find(req.uri().path(), &method).unwrap(); // todo 错误处理
            let request: Request<State> = crate::request::Request::from_http(req, state, params);
            // 取出下一个执行的中间件并链式执行
            let chain = Chain {
                handler,
                next: &middlewares,
            };
            let response = chain.call(request).await;
            Ok(response.into())
        };
        Box::pin(future)
    }
}

/// MakeService实现了Service trait
/// 并且满足了MakeServiceRef trait的要求: S满足HttpService
/// 所以满足MakeServiceRef<I::Conn, Body, ResBody = B>
/// ```
/// use hyper::service::Service;
/// impl<T, Target, E, ME, S, F, IB, OB> MakeServiceRef<Target, IB> for T
/// where
///     T: for<'a> Service<&'a Target, Error = ME, Response = S, Future = F>,
///     S: HttpService<IB, ResBody = OB, Error = E>{}
/// ```
///
pub struct MakeService<State>(App<State>);

impl<T, State: Clone> Service<T> for MakeService<State> {
    type Response = App<State>;
    type Error = std::io::Error;
    type Future = future::Ready<std::result::Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        future::ok(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
