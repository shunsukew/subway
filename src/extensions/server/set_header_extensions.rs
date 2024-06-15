use hyper::body::Bytes;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Layer, Service};
use jsonrpsee::core::{
    http_helpers::{Request as HttpRequest, Response as HttpResponse},
    BoxError,
};
use futures_util::{Future, TryFutureExt};

#[derive(Debug, Clone)]
pub struct SetHeaderExtensionsLayer {}

impl SetHeaderExtensionsLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S> Layer<S> for SetHeaderExtensionsLayer {
    type Service = SetHeaderExtensions<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SetHeaderExtensions::new(inner)
    }
}

#[derive(Debug, Clone)]
pub struct SetHeaderExtensions<S>{
    inner: S,
}

impl<S> SetHeaderExtensions<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<HttpRequest<B>> for SetHeaderExtensions<S>
where
    S: Service<HttpRequest<B>, Response = HttpResponse>,
    S::Response: 'static,
    S::Error: Into<BoxError> + 'static,
    S::Future: Send + 'static,
    B: http_body::Body<Data = Bytes> + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, mut req: HttpRequest<B>) -> Self::Future {
        // By default jsonrpsee sets only connection_id to extensions.
        let headers = req.headers().clone();
        req.extensions_mut().insert(headers);

        Box::pin(self.inner.call(req).map_err(Into::into))
    }
}
