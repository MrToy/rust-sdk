// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
#[derive(Debug, Clone, Default)]
#[doc = "调用 API 所用的路径参数"]
pub struct PathParams {
    r#entry: Option<std::borrow::Cow<'static, str>>,
    r#to_ia_after_days: Option<std::borrow::Cow<'static, str>>,
    r#to_archive_after_days: Option<std::borrow::Cow<'static, str>>,
    r#delete_after_days: Option<std::borrow::Cow<'static, str>>,
    extended_segments: Vec<std::borrow::Cow<'static, str>>,
}
impl PathParams {
    #[inline]
    #[must_use]
    pub fn push_segment(mut self, segment: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        self.extended_segments.push(segment.into());
        self
    }
    fn build(self) -> Vec<std::borrow::Cow<'static, str>> {
        let mut all_segments: Vec<_> = Default::default();
        if let Some(segment) = self.r#entry {
            all_segments.push(segment);
        }
        if let Some(segment) = self.r#to_ia_after_days {
            all_segments.push(std::borrow::Cow::Borrowed("toIAAfterDays"));
            all_segments.push(segment);
        }
        if let Some(segment) = self.r#to_archive_after_days {
            all_segments.push(std::borrow::Cow::Borrowed("toARCHIVEAfterDays"));
            all_segments.push(segment);
        }
        if let Some(segment) = self.r#delete_after_days {
            all_segments.push(std::borrow::Cow::Borrowed("deleteAfterDays"));
            all_segments.push(segment);
        }
        all_segments.extend(self.extended_segments);
        all_segments
    }
}
impl PathParams {
    #[inline]
    #[must_use]
    #[doc = "指定目标对象空间与目标对象名称"]
    pub fn set_entry_as_str(mut self, value: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        self.r#entry = Some(qiniu_utils::base64::urlsafe(value.into().as_bytes()).into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToIAAfterDays 转换到低频存储类型，设置为 -1 表示取消已设置的转低频存储的生命周期规则"]
    pub fn set_to_ia_after_days_as_i8(mut self, value: i8) -> Self {
        self.r#to_ia_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToIAAfterDays 转换到低频存储类型，设置为 -1 表示取消已设置的转低频存储的生命周期规则"]
    pub fn set_to_ia_after_days_as_i16(mut self, value: i16) -> Self {
        self.r#to_ia_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToIAAfterDays 转换到低频存储类型，设置为 -1 表示取消已设置的转低频存储的生命周期规则"]
    pub fn set_to_ia_after_days_as_i32(mut self, value: i32) -> Self {
        self.r#to_ia_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToIAAfterDays 转换到低频存储类型，设置为 -1 表示取消已设置的转低频存储的生命周期规则"]
    pub fn set_to_ia_after_days_as_i64(mut self, value: i64) -> Self {
        self.r#to_ia_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToIAAfterDays 转换到低频存储类型，设置为 -1 表示取消已设置的转低频存储的生命周期规则"]
    pub fn set_to_ia_after_days_as_isize(mut self, value: isize) -> Self {
        self.r#to_ia_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToIAAfterDays 转换到低频存储类型，设置为 -1 表示取消已设置的转低频存储的生命周期规则"]
    pub fn set_to_ia_after_days_as_u8(mut self, value: u8) -> Self {
        self.r#to_ia_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToIAAfterDays 转换到低频存储类型，设置为 -1 表示取消已设置的转低频存储的生命周期规则"]
    pub fn set_to_ia_after_days_as_u16(mut self, value: u16) -> Self {
        self.r#to_ia_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToIAAfterDays 转换到低频存储类型，设置为 -1 表示取消已设置的转低频存储的生命周期规则"]
    pub fn set_to_ia_after_days_as_u32(mut self, value: u32) -> Self {
        self.r#to_ia_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToIAAfterDays 转换到低频存储类型，设置为 -1 表示取消已设置的转低频存储的生命周期规则"]
    pub fn set_to_ia_after_days_as_u64(mut self, value: u64) -> Self {
        self.r#to_ia_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToIAAfterDays 转换到低频存储类型，设置为 -1 表示取消已设置的转低频存储的生命周期规则"]
    pub fn set_to_ia_after_days_as_usize(mut self, value: usize) -> Self {
        self.r#to_ia_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToARCHIVEAfterDays 转换到 归档存储类型， 设置为 -1 表示取消已设置的转归档存储的生命周期规则"]
    pub fn set_to_archive_after_days_as_i8(mut self, value: i8) -> Self {
        self.r#to_archive_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToARCHIVEAfterDays 转换到 归档存储类型， 设置为 -1 表示取消已设置的转归档存储的生命周期规则"]
    pub fn set_to_archive_after_days_as_i16(mut self, value: i16) -> Self {
        self.r#to_archive_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToARCHIVEAfterDays 转换到 归档存储类型， 设置为 -1 表示取消已设置的转归档存储的生命周期规则"]
    pub fn set_to_archive_after_days_as_i32(mut self, value: i32) -> Self {
        self.r#to_archive_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToARCHIVEAfterDays 转换到 归档存储类型， 设置为 -1 表示取消已设置的转归档存储的生命周期规则"]
    pub fn set_to_archive_after_days_as_i64(mut self, value: i64) -> Self {
        self.r#to_archive_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToARCHIVEAfterDays 转换到 归档存储类型， 设置为 -1 表示取消已设置的转归档存储的生命周期规则"]
    pub fn set_to_archive_after_days_as_isize(mut self, value: isize) -> Self {
        self.r#to_archive_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToARCHIVEAfterDays 转换到 归档存储类型， 设置为 -1 表示取消已设置的转归档存储的生命周期规则"]
    pub fn set_to_archive_after_days_as_u8(mut self, value: u8) -> Self {
        self.r#to_archive_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToARCHIVEAfterDays 转换到 归档存储类型， 设置为 -1 表示取消已设置的转归档存储的生命周期规则"]
    pub fn set_to_archive_after_days_as_u16(mut self, value: u16) -> Self {
        self.r#to_archive_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToARCHIVEAfterDays 转换到 归档存储类型， 设置为 -1 表示取消已设置的转归档存储的生命周期规则"]
    pub fn set_to_archive_after_days_as_u32(mut self, value: u32) -> Self {
        self.r#to_archive_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToARCHIVEAfterDays 转换到 归档存储类型， 设置为 -1 表示取消已设置的转归档存储的生命周期规则"]
    pub fn set_to_archive_after_days_as_u64(mut self, value: u64) -> Self {
        self.r#to_archive_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 ToARCHIVEAfterDays 转换到 归档存储类型， 设置为 -1 表示取消已设置的转归档存储的生命周期规则"]
    pub fn set_to_archive_after_days_as_usize(mut self, value: usize) -> Self {
        self.r#to_archive_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 DeleteAfterDays 过期删除，删除后不可恢复，设置为 -1 表示取消已设置的过期删除的生命周期规则"]
    pub fn set_delete_after_days_as_i8(mut self, value: i8) -> Self {
        self.r#delete_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 DeleteAfterDays 过期删除，删除后不可恢复，设置为 -1 表示取消已设置的过期删除的生命周期规则"]
    pub fn set_delete_after_days_as_i16(mut self, value: i16) -> Self {
        self.r#delete_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 DeleteAfterDays 过期删除，删除后不可恢复，设置为 -1 表示取消已设置的过期删除的生命周期规则"]
    pub fn set_delete_after_days_as_i32(mut self, value: i32) -> Self {
        self.r#delete_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 DeleteAfterDays 过期删除，删除后不可恢复，设置为 -1 表示取消已设置的过期删除的生命周期规则"]
    pub fn set_delete_after_days_as_i64(mut self, value: i64) -> Self {
        self.r#delete_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 DeleteAfterDays 过期删除，删除后不可恢复，设置为 -1 表示取消已设置的过期删除的生命周期规则"]
    pub fn set_delete_after_days_as_isize(mut self, value: isize) -> Self {
        self.r#delete_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 DeleteAfterDays 过期删除，删除后不可恢复，设置为 -1 表示取消已设置的过期删除的生命周期规则"]
    pub fn set_delete_after_days_as_u8(mut self, value: u8) -> Self {
        self.r#delete_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 DeleteAfterDays 过期删除，删除后不可恢复，设置为 -1 表示取消已设置的过期删除的生命周期规则"]
    pub fn set_delete_after_days_as_u16(mut self, value: u16) -> Self {
        self.r#delete_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 DeleteAfterDays 过期删除，删除后不可恢复，设置为 -1 表示取消已设置的过期删除的生命周期规则"]
    pub fn set_delete_after_days_as_u32(mut self, value: u32) -> Self {
        self.r#delete_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 DeleteAfterDays 过期删除，删除后不可恢复，设置为 -1 表示取消已设置的过期删除的生命周期规则"]
    pub fn set_delete_after_days_as_u64(mut self, value: u64) -> Self {
        self.r#delete_after_days = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "指定文件上传后在设置的 DeleteAfterDays 过期删除，删除后不可恢复，设置为 -1 表示取消已设置的过期删除的生命周期规则"]
    pub fn set_delete_after_days_as_usize(mut self, value: usize) -> Self {
        self.r#delete_after_days = Some(value.to_string().into());
        self
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "获取 API 所用的响应体参数"]
pub struct ResponseBody(serde_json::Value);
impl ResponseBody {
    #[allow(dead_code)]
    pub(crate) fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}
impl From<ResponseBody> for serde_json::Value {
    #[inline]
    fn from(val: ResponseBody) -> Self {
        val.0
    }
}
impl std::convert::AsRef<serde_json::Value> for ResponseBody {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl std::convert::AsMut<serde_json::Value> for ResponseBody {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.0
    }
}
#[derive(Debug, Clone)]
pub struct Client<'client>(&'client qiniu_http_client::HttpClient);
impl<'client> Client<'client> {
    pub(super) fn new(http_client: &'client qiniu_http_client::HttpClient) -> Self {
        Self(http_client)
    }
}
impl<'client> Client<'client> {
    #[inline]
    pub fn new_request<E: qiniu_http_client::EndpointsProvider + 'client>(
        &self,
        endpoints_provider: E,
        path_params: PathParams,
        credential: impl qiniu_http_client::credential::CredentialProvider + std::clone::Clone + 'static,
    ) -> SyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self
                .0
                .post(&[qiniu_http_client::ServiceName::Rs], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::v2(credential));
            builder.idempotent(qiniu_http_client::Idempotent::Always);
            builder.path(crate::base_utils::join_path(
                "/lifecycle",
                "",
                path_params.build(),
            ));
            builder.accept_json();
            builder
        })
    }
    #[inline]
    #[cfg(feature = "async")]
    pub fn new_async_request<E: qiniu_http_client::EndpointsProvider + 'client>(
        &self,
        endpoints_provider: E,
        path_params: PathParams,
        credential: impl qiniu_http_client::credential::CredentialProvider + std::clone::Clone + 'static,
    ) -> AsyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self
                .0
                .async_post(&[qiniu_http_client::ServiceName::Rs], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::v2(credential));
            builder.idempotent(qiniu_http_client::Idempotent::Always);
            builder.path(crate::base_utils::join_path(
                "/lifecycle",
                "",
                path_params.build(),
            ));
            builder.accept_json();
            builder
        })
    }
}
#[derive(Debug)]
pub struct RequestBuilder<'req, B: 'req, E: 'req>(qiniu_http_client::RequestBuilder<'req, B, E>);
pub type SyncRequestBuilder<'req, E> =
    RequestBuilder<'req, qiniu_http_client::SyncRequestBody<'req>, E>;
#[cfg(feature = "async")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
pub type AsyncRequestBuilder<'req, E> =
    RequestBuilder<'req, qiniu_http_client::AsyncRequestBody<'req>, E>;
impl<'req, B: 'req, E: 'req> RequestBuilder<'req, B, E> {
    #[inline]
    pub fn use_https(&mut self, use_https: bool) -> &mut Self {
        self.0.use_https(use_https);
        self
    }
    #[inline]
    pub fn version(&mut self, version: qiniu_http_client::http::Version) -> &mut Self {
        self.0.version(version);
        self
    }
    #[inline]
    pub fn headers(
        &mut self,
        headers: impl Into<std::borrow::Cow<'req, qiniu_http_client::http::HeaderMap>>,
    ) -> &mut Self {
        self.0.headers(headers);
        self
    }
    #[inline]
    pub fn query_pairs(
        &mut self,
        query_pairs: impl Into<qiniu_http_client::QueryPairs<'req>>,
    ) -> &mut Self {
        self.0.query_pairs(query_pairs);
        self
    }
    #[inline]
    pub fn extensions(&mut self, extensions: qiniu_http_client::http::Extensions) -> &mut Self {
        self.0.extensions(extensions);
        self
    }
    #[inline]
    pub fn add_extension<T: Send + Sync + 'static>(&mut self, val: T) -> &mut Self {
        self.0.add_extension(val);
        self
    }
    #[inline]
    pub fn on_uploading_progress(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                &qiniu_http_client::http::TransferProgressInfo,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_uploading_progress(callback);
        self
    }
    #[inline]
    pub fn on_receive_response_status(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                qiniu_http_client::http::StatusCode,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_receive_response_status(callback);
        self
    }
    #[inline]
    pub fn on_receive_response_header(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                &qiniu_http_client::http::HeaderName,
                &qiniu_http_client::http::HeaderValue,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_receive_response_header(callback);
        self
    }
    #[inline]
    pub fn on_to_resolve_domain(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &str,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_to_resolve_domain(callback);
        self
    }
    #[inline]
    pub fn on_domain_resolved(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &str,
                &qiniu_http_client::ResolveAnswers,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_domain_resolved(callback);
        self
    }
    #[inline]
    pub fn on_to_choose_ips(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &[qiniu_http_client::IpAddrWithPort],
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_to_choose_ips(callback);
        self
    }
    #[inline]
    pub fn on_ips_chosen(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &[qiniu_http_client::IpAddrWithPort],
                &[qiniu_http_client::IpAddrWithPort],
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_ips_chosen(callback);
        self
    }
    #[inline]
    pub fn on_before_request_signed(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_before_request_signed(callback);
        self
    }
    #[inline]
    pub fn on_after_request_signed(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_after_request_signed(callback);
        self
    }
    #[inline]
    pub fn on_error(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                &qiniu_http_client::ResponseError,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_error(callback);
        self
    }
    #[inline]
    pub fn on_before_backoff(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                std::time::Duration,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_before_backoff(callback);
        self
    }
    #[inline]
    pub fn on_after_backoff(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                std::time::Duration,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_after_backoff(callback);
        self
    }
    #[inline]
    pub fn parts_mut(&mut self) -> &mut qiniu_http_client::RequestBuilderParts<'req> {
        self.0.parts_mut()
    }
}
impl<'req, E: qiniu_http_client::EndpointsProvider + Clone + 'req> SyncRequestBuilder<'req, E> {
    pub fn call(
        &mut self,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = &mut self.0;
        let response = request.call()?;
        let parsed = response.parse_json()?;
        Ok(parsed)
    }
}
#[cfg(feature = "async")]
impl<'req, E: qiniu_http_client::EndpointsProvider + Clone + 'req> AsyncRequestBuilder<'req, E> {
    pub async fn call(
        &mut self,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = &mut self.0;
        let response = request.call().await?;
        let parsed = response.parse_json().await?;
        Ok(parsed)
    }
}
