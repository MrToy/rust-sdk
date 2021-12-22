use super::{
    super::{
        BackoffOptions, RequestParts, RequestRetrierOptions, ResponseError, RetriedStatsInfo,
        RetryDecision, SimplifiedCallbackContext, SyncResponse,
    },
    error::TryError,
    utils::{
        call_after_backoff_callbacks, call_before_backoff_callbacks, call_error_callbacks,
        call_response_callbacks, judge,
    },
};
use log::error;
use qiniu_http::{RequestParts as HttpRequestParts, Reset, SyncRequest as SyncHttpRequest};
use std::{result::Result, thread::sleep, time::Duration};

pub(super) fn send_http_request(
    http_request: &mut SyncHttpRequest<'_>,
    parts: &RequestParts<'_>,
    retried: &mut RetriedStatsInfo,
) -> Result<SyncResponse, TryError> {
    loop {
        let response = parts
            .http_client()
            .http_caller()
            .call(http_request)
            .map_err(ResponseError::from)
            .and_then(|response| {
                call_response_callbacks(parts, http_request, retried, response.parts())
                    .map(|_| response)
            })
            .map(SyncResponse::new)
            .and_then(|response| judge(response, retried))
            .map_err(|err| handle_response_error(err, http_request, parts, retried));
        match response {
            Ok(response) => {
                return Ok(response);
            }
            Err(err) => {
                call_error_callbacks(parts, http_request, retried, err.response_error())?;
                if need_backoff(&err) {
                    backoff(http_request, parts, retried, &err)?;
                    if need_retry_after_backoff(&err) {
                        continue;
                    }
                }
                reset_body_if_needed(http_request, &err);
                return Err(err);
            }
        }
    }

    fn backoff(
        http_request: &mut SyncHttpRequest<'_>,
        parts: &RequestParts<'_>,
        retried: &mut RetriedStatsInfo,
        err: &TryError,
    ) -> Result<(), TryError> {
        let delay = parts
            .http_client()
            .backoff()
            .time(
                http_request,
                &BackoffOptions::new(err.retry_decision(), err.response_error(), retried),
            )
            .duration();
        call_before_backoff_callbacks(parts, http_request, retried, delay)?;
        if delay > Duration::new(0, 0) {
            sleep(delay);
        }
        call_after_backoff_callbacks(parts, http_request, retried, delay)?;
        Ok(())
    }

    fn reset_body_if_needed(http_request: &mut SyncHttpRequest<'_>, err: &TryError) {
        match err.retry_decision() {
            RetryDecision::DontRetry => {}
            _ => {
                if let Err(err) = http_request.body_mut().reset() {
                    error!("Failed to reset http request body: {}", err)
                }
            }
        }
    }
}

fn need_backoff(err: &TryError) -> bool {
    matches!(
        err.retry_decision(),
        RetryDecision::RetryRequest | RetryDecision::Throttled | RetryDecision::TryNextServer
    )
}

fn need_retry_after_backoff(err: &TryError) -> bool {
    matches!(
        err.retry_decision(),
        RetryDecision::RetryRequest | RetryDecision::Throttled
    )
}

fn handle_response_error(
    response_error: ResponseError,
    http_parts: &mut HttpRequestParts,
    parts: &RequestParts<'_>,
    retried: &mut RetriedStatsInfo,
) -> TryError {
    let retry_result = parts.http_client().request_retrier().retry(
        http_parts,
        &RequestRetrierOptions::new(parts.idempotent(), &response_error, retried),
    );
    retried.increase();
    TryError::new(response_error, retry_result)
}

#[cfg(feature = "async")]
mod async_send {
    use super::{
        super::{super::AsyncResponse, utils::async_judge},
        *,
    };
    use async_std::task::block_on;
    use futures_timer::Delay as AsyncDelay;
    use qiniu_http::{AsyncRequest as AsyncHttpRequest, AsyncReset};

    pub(in super::super) async fn async_send_http_request(
        http_request: &mut AsyncHttpRequest<'_>,
        parts: &RequestParts<'_>,
        retried: &mut RetriedStatsInfo,
    ) -> Result<AsyncResponse, TryError> {
        loop {
            let response = parts
                .http_client()
                .http_caller()
                .async_call(http_request)
                .await
                .map_err(ResponseError::from)
                .and_then(|response| {
                    call_response_callbacks(parts, http_request, retried, response.parts())
                        .map(|_| response)
                })
                .map(AsyncResponse::new)
                .and_then(|err| block_on(async { async_judge(err, retried).await }))
                .map_err(|err| handle_response_error(err, http_request, parts, retried));
            match response {
                Ok(response) => {
                    return Ok(response);
                }
                Err(err) => {
                    call_error_callbacks(parts, http_request, retried, err.response_error())?;
                    if need_backoff(&err) {
                        backoff(http_request, parts, retried, &err).await?;
                        if need_retry_after_backoff(&err) {
                            continue;
                        }
                    }
                    reset_body_if_needed(http_request, &err).await;
                    return Err(err);
                }
            }
        }

        async fn backoff(
            http_request: &mut AsyncHttpRequest<'_>,
            parts: &RequestParts<'_>,
            retried: &mut RetriedStatsInfo,
            err: &TryError,
        ) -> Result<(), TryError> {
            let delay = parts
                .http_client()
                .backoff()
                .time(
                    http_request,
                    &BackoffOptions::new(err.retry_decision(), err.response_error(), retried),
                )
                .duration();
            call_before_backoff_callbacks(parts, http_request, retried, delay)?;
            if delay > Duration::new(0, 0) {
                async_sleep(delay).await;
            }
            call_after_backoff_callbacks(parts, http_request, retried, delay)?;
            Ok(())
        }

        async fn reset_body_if_needed(http_request: &mut AsyncHttpRequest<'_>, err: &TryError) {
            match err.retry_decision() {
                RetryDecision::DontRetry => {}
                _ => {
                    if let Err(err) = http_request.body_mut().reset().await {
                        error!("Failed to reset http request body: {}", err)
                    }
                }
            }
        }

        async fn async_sleep(dur: Duration) {
            AsyncDelay::new(dur).await
        }
    }
}

#[cfg(feature = "async")]
pub(super) use async_send::*;
