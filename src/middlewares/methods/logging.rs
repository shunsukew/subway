use async_trait::async_trait;
use opentelemetry::trace::FutureExt;

use crate::{
    middlewares::{CallRequest, CallResult, Middleware, MiddlewareBuilder, NextFn, RpcMethod, TRACER},
    utils::{TypeRegistry, TypeRegistryRef},
};

pub struct LoggingMiddleware {}

impl LoggingMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl MiddlewareBuilder<RpcMethod, CallRequest, CallResult> for LoggingMiddleware {
    async fn build(
        _method: &RpcMethod,
        _extensions: &TypeRegistryRef,
    ) -> Option<Box<dyn Middleware<CallRequest, CallResult>>> {
        Some(Box::new(LoggingMiddleware::new()))
    }
}

// What to log
// - http header value
// - request
//   - method
//   - params
// - response
//   - status
//   - body
//   - response time

#[async_trait]
impl Middleware<CallRequest, CallResult> for LoggingMiddleware {
    async fn call(
        &self,
        request: CallRequest,
        context: TypeRegistry,
        next: NextFn<CallRequest, CallResult>,
    ) -> CallResult {
        println!("Logging middleware called");

        async move {
            let result = next(request, context).await;

            result
        }
        .with_context(TRACER.context("logging"))
        .await
    }
}
