use super::{
    super::{Idempotent, ResponseError, RetriedStatsInfo},
    RequestRetrier, RetryResult,
};
use qiniu_http::Request as HTTPRequest;
use std::any::Any;

#[derive(Default, Copy, Clone, Debug)]
pub struct NeverRetrier;

impl RequestRetrier for NeverRetrier {
    #[inline]
    fn retry(
        &self,
        _request: &mut HTTPRequest,
        _idempotent: Idempotent,
        _response_error: &ResponseError,
        _retried: &RetriedStatsInfo,
    ) -> RetryResult {
        RetryResult::DontRetry
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_request_retrier(&self) -> &dyn RequestRetrier {
        self
    }
}