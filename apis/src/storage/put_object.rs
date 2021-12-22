// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
pub mod sync_part {
    #[derive(Debug, Default)]
    #[doc = "调用 API 所用的请求体参数"]
    pub struct RequestBody {
        multipart: qiniu_http_client::SyncMultipart,
    }
    impl RequestBody {
        #[inline]
        #[must_use]
        pub fn add_part(
            mut self,
            name: impl Into<qiniu_http_client::FieldName>,
            part: qiniu_http_client::SyncPart,
        ) -> Self {
            self.multipart = self.multipart.add_part(name.into(), part);
            self
        }
        fn build(self) -> qiniu_http_client::SyncMultipart {
            self.multipart
        }
    }
    impl From<RequestBody> for qiniu_http_client::SyncMultipart {
        #[inline]
        fn from(parts: RequestBody) -> Self {
            parts.build()
        }
    }
    impl RequestBody {
        #[inline]
        #[must_use]
        #[doc = "对象名称，如果不传入，则通过上传策略中的 `saveKey` 字段决定，如果 `saveKey` 也没有置顶，则使用对象的哈希值"]
        pub fn set_object_name(self, value: impl Into<std::borrow::Cow<'static, str>>) -> Self {
            self.add_part("key", qiniu_http_client::SyncPart::text(value))
        }
        #[inline]
        #[doc = "上传凭证"]
        pub fn set_upload_token(
            self,
            token: &dyn qiniu_http_client::upload_token::UploadTokenProvider,
        ) -> std::io::Result<Self> {
            Ok(self.add_part(
                "token",
                qiniu_http_client::SyncPart::text(String::from(
                    token.to_token_string(&Default::default())?,
                )),
            ))
        }
        #[inline]
        #[must_use]
        #[doc = "上传内容的 CRC32 校验码，如果指定此值，则七牛服务器会使用此值进行内容检验"]
        pub fn set_crc_32(self, value: impl Into<std::borrow::Cow<'static, str>>) -> Self {
            self.add_part("crc32", qiniu_http_client::SyncPart::text(value))
        }
        #[inline]
        #[must_use]
        #[doc = "上传文件的内容"]
        pub fn set_file_as_reader(
            self,
            reader: impl std::io::Read + 'static,
            metadata: qiniu_http_client::PartMetadata,
        ) -> Self {
            self.add_part(
                "file",
                qiniu_http_client::SyncPart::stream(reader).metadata(metadata),
            )
        }
        #[inline]
        #[must_use]
        #[doc = "上传文件的内容"]
        pub fn set_file_as_bytes(
            self,
            bytes: impl Into<std::borrow::Cow<'static, [u8]>>,
            metadata: qiniu_http_client::PartMetadata,
        ) -> Self {
            self.add_part(
                "file",
                qiniu_http_client::SyncPart::bytes(bytes).metadata(metadata),
            )
        }
        #[inline]
        #[doc = "上传文件的内容"]
        pub fn set_file_as_file_path<S: AsRef<std::ffi::OsStr> + ?Sized>(
            self,
            path: &S,
        ) -> std::io::Result<Self> {
            Ok(self.add_part(
                "file",
                qiniu_http_client::SyncPart::file_path(std::path::Path::new(path))?,
            ))
        }
        #[inline]
        #[must_use]
        #[doc = "自定义元数据（需要以 `x-qn-meta-` 作为前缀）或自定义变量（需要以 `x:` 作为前缀）"]
        pub fn append_custom_data(
            self,
            key: impl Into<qiniu_http_client::FieldName>,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> Self {
            self.add_part(key, qiniu_http_client::SyncPart::text(value))
        }
    }
}
#[cfg(feature = "async")]
pub mod async_part {
    #[derive(Debug, Default)]
    #[doc = "调用 API 所用的请求体参数"]
    pub struct RequestBody {
        multipart: qiniu_http_client::AsyncMultipart,
    }
    impl RequestBody {
        #[inline]
        #[must_use]
        pub fn add_part(
            mut self,
            name: impl Into<qiniu_http_client::FieldName>,
            part: qiniu_http_client::AsyncPart,
        ) -> Self {
            self.multipart = self.multipart.add_part(name.into(), part);
            self
        }
        fn build(self) -> qiniu_http_client::AsyncMultipart {
            self.multipart
        }
    }
    impl From<RequestBody> for qiniu_http_client::AsyncMultipart {
        #[inline]
        fn from(parts: RequestBody) -> Self {
            parts.build()
        }
    }
    impl RequestBody {
        #[inline]
        #[must_use]
        #[doc = "对象名称，如果不传入，则通过上传策略中的 `saveKey` 字段决定，如果 `saveKey` 也没有置顶，则使用对象的哈希值"]
        pub fn set_object_name(self, value: impl Into<std::borrow::Cow<'static, str>>) -> Self {
            self.add_part("key", qiniu_http_client::AsyncPart::text(value))
        }
        #[inline]
        #[doc = "上传凭证"]
        pub async fn set_upload_token(
            self,
            token: &dyn qiniu_http_client::upload_token::UploadTokenProvider,
        ) -> std::io::Result<Self> {
            Ok(self.add_part(
                "token",
                qiniu_http_client::AsyncPart::text(String::from(
                    token.async_to_token_string(&Default::default()).await?,
                )),
            ))
        }
        #[inline]
        #[must_use]
        #[doc = "上传内容的 CRC32 校验码，如果指定此值，则七牛服务器会使用此值进行内容检验"]
        pub fn set_crc_32(self, value: impl Into<std::borrow::Cow<'static, str>>) -> Self {
            self.add_part("crc32", qiniu_http_client::AsyncPart::text(value))
        }
        #[inline]
        #[must_use]
        #[doc = "上传文件的内容"]
        pub fn set_file_as_reader(
            self,
            reader: impl futures::io::AsyncRead + Send + Unpin + 'static,
            metadata: qiniu_http_client::PartMetadata,
        ) -> Self {
            self.add_part(
                "file",
                qiniu_http_client::AsyncPart::stream(reader).metadata(metadata),
            )
        }
        #[inline]
        #[must_use]
        #[doc = "上传文件的内容"]
        pub fn set_file_as_bytes(
            self,
            bytes: impl Into<std::borrow::Cow<'static, [u8]>>,
            metadata: qiniu_http_client::PartMetadata,
        ) -> Self {
            self.add_part(
                "file",
                qiniu_http_client::AsyncPart::bytes(bytes).metadata(metadata),
            )
        }
        #[inline]
        #[doc = "上传文件的内容"]
        pub async fn set_file_as_file_path<S: AsRef<std::ffi::OsStr> + ?Sized>(
            self,
            path: &S,
        ) -> std::io::Result<Self> {
            Ok(self.add_part(
                "file",
                qiniu_http_client::AsyncPart::file_path(async_std::path::Path::new(path)).await?,
            ))
        }
        #[inline]
        #[must_use]
        #[doc = "自定义元数据（需要以 `x-qn-meta-` 作为前缀）或自定义变量（需要以 `x:` 作为前缀）"]
        pub fn append_custom_data(
            self,
            key: impl Into<qiniu_http_client::FieldName>,
            value: impl Into<std::borrow::Cow<'static, str>>,
        ) -> Self {
            self.add_part(key, qiniu_http_client::AsyncPart::text(value))
        }
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
    ) -> SyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self
                .0
                .post(&[qiniu_http_client::ServiceName::Up], endpoints_provider);
            builder.idempotent(qiniu_http_client::Idempotent::Default);
            builder.path("");
            builder.accept_json();
            builder
        })
    }
    #[inline]
    #[cfg(feature = "async")]
    pub fn new_async_request<E: qiniu_http_client::EndpointsProvider + 'client>(
        &self,
        endpoints_provider: E,
    ) -> AsyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self
                .0
                .async_post(&[qiniu_http_client::ServiceName::Up], endpoints_provider);
            builder.idempotent(qiniu_http_client::Idempotent::Default);
            builder.path("");
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
    pub fn on_response(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                &qiniu_http_client::http::ResponseParts,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_response(callback);
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
        body: sync_part::RequestBody,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = self.0.multipart(body)?;
        let response = request.call()?;
        let parsed = response.parse_json()?;
        Ok(parsed)
    }
}
#[cfg(feature = "async")]
impl<'req, E: qiniu_http_client::EndpointsProvider + Clone + 'req> AsyncRequestBuilder<'req, E> {
    pub async fn call(
        &mut self,
        body: async_part::RequestBody,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = self.0.multipart(body).await?;
        let response = request.call().await?;
        let parsed = response.parse_json().await?;
        Ok(parsed)
    }
}
