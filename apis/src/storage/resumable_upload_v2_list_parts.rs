// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
#[derive(Debug, Clone, Default)]
#[doc = "调用 API 所用的路径参数"]
pub struct PathParams {
    r#bucket_name: Option<std::borrow::Cow<'static, str>>,
    r#object_name: Option<std::borrow::Cow<'static, str>>,
    r#upload_id: Option<std::borrow::Cow<'static, str>>,
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
        if let Some(segment) = self.r#bucket_name {
            all_segments.push(segment);
        }
        all_segments.push(std::borrow::Cow::Borrowed("objects"));
        all_segments.push(
            self.r#object_name
                .unwrap_or(std::borrow::Cow::Borrowed("~")),
        );
        if let Some(segment) = self.r#upload_id {
            all_segments.push(std::borrow::Cow::Borrowed("uploads"));
            all_segments.push(segment);
        }
        all_segments.extend(self.extended_segments);
        all_segments
    }
}
impl PathParams {
    #[inline]
    #[must_use]
    #[doc = "存储空间名称"]
    pub fn set_bucket_name_as_str(
        mut self,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.r#bucket_name = Some(value.into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "对象名称"]
    pub fn set_object_name_as_str(
        mut self,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.r#object_name = Some(qiniu_utils::base64::urlsafe(value.into().as_bytes()).into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "在服务端申请的 Multipart Upload 任务 id"]
    pub fn set_upload_id_as_str(
        mut self,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.r#upload_id = Some(value.into());
        self
    }
}
#[derive(Debug, Clone, Default)]
#[doc = "调用 API 所用的 URL 查询参数"]
pub struct QueryParams<'a> {
    map: indexmap::IndexMap<
        qiniu_http_client::QueryPairKey<'a>,
        qiniu_http_client::QueryPairValue<'a>,
    >,
}
impl<'a> QueryParams<'a> {
    #[inline]
    #[must_use]
    pub fn insert(
        mut self,
        query_pair_key: qiniu_http_client::QueryPairKey<'a>,
        query_pair_value: qiniu_http_client::QueryPairValue<'a>,
    ) -> Self {
        self.map.insert(query_pair_key, query_pair_value);
        self
    }
    fn build(self) -> qiniu_http_client::QueryPairs<'a> {
        qiniu_http_client::QueryPairs::from_iter(self.map)
    }
}
impl<'a> From<QueryParams<'a>> for qiniu_http_client::QueryPairs<'a> {
    #[inline]
    fn from(map: QueryParams<'a>) -> Self {
        map.build()
    }
}
impl<'a> QueryParams<'a> {
    #[inline]
    #[must_use]
    #[doc = "响应中的最大分片数目。默认值：1000，最大值：1000"]
    pub fn set_max_parts_as_i8(self, value: i8) -> Self {
        self.insert("max-parts".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "响应中的最大分片数目。默认值：1000，最大值：1000"]
    pub fn set_max_parts_as_i16(self, value: i16) -> Self {
        self.insert("max-parts".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "响应中的最大分片数目。默认值：1000，最大值：1000"]
    pub fn set_max_parts_as_i32(self, value: i32) -> Self {
        self.insert("max-parts".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "响应中的最大分片数目。默认值：1000，最大值：1000"]
    pub fn set_max_parts_as_i64(self, value: i64) -> Self {
        self.insert("max-parts".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "响应中的最大分片数目。默认值：1000，最大值：1000"]
    pub fn set_max_parts_as_isize(self, value: isize) -> Self {
        self.insert("max-parts".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "响应中的最大分片数目。默认值：1000，最大值：1000"]
    pub fn set_max_parts_as_u8(self, value: u8) -> Self {
        self.insert("max-parts".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "响应中的最大分片数目。默认值：1000，最大值：1000"]
    pub fn set_max_parts_as_u16(self, value: u16) -> Self {
        self.insert("max-parts".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "响应中的最大分片数目。默认值：1000，最大值：1000"]
    pub fn set_max_parts_as_u32(self, value: u32) -> Self {
        self.insert("max-parts".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "响应中的最大分片数目。默认值：1000，最大值：1000"]
    pub fn set_max_parts_as_u64(self, value: u64) -> Self {
        self.insert("max-parts".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "响应中的最大分片数目。默认值：1000，最大值：1000"]
    pub fn set_max_parts_as_usize(self, value: usize) -> Self {
        self.insert("max-parts".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "指定列举的起始位置，只有 partNumber 值大于该参数的分片会被列出"]
    pub fn set_part_number_marker_as_i8(self, value: i8) -> Self {
        self.insert("part-number_marker".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "指定列举的起始位置，只有 partNumber 值大于该参数的分片会被列出"]
    pub fn set_part_number_marker_as_i16(self, value: i16) -> Self {
        self.insert("part-number_marker".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "指定列举的起始位置，只有 partNumber 值大于该参数的分片会被列出"]
    pub fn set_part_number_marker_as_i32(self, value: i32) -> Self {
        self.insert("part-number_marker".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "指定列举的起始位置，只有 partNumber 值大于该参数的分片会被列出"]
    pub fn set_part_number_marker_as_i64(self, value: i64) -> Self {
        self.insert("part-number_marker".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "指定列举的起始位置，只有 partNumber 值大于该参数的分片会被列出"]
    pub fn set_part_number_marker_as_isize(self, value: isize) -> Self {
        self.insert("part-number_marker".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "指定列举的起始位置，只有 partNumber 值大于该参数的分片会被列出"]
    pub fn set_part_number_marker_as_u8(self, value: u8) -> Self {
        self.insert("part-number_marker".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "指定列举的起始位置，只有 partNumber 值大于该参数的分片会被列出"]
    pub fn set_part_number_marker_as_u16(self, value: u16) -> Self {
        self.insert("part-number_marker".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "指定列举的起始位置，只有 partNumber 值大于该参数的分片会被列出"]
    pub fn set_part_number_marker_as_u32(self, value: u32) -> Self {
        self.insert("part-number_marker".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "指定列举的起始位置，只有 partNumber 值大于该参数的分片会被列出"]
    pub fn set_part_number_marker_as_u64(self, value: u64) -> Self {
        self.insert("part-number_marker".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "指定列举的起始位置，只有 partNumber 值大于该参数的分片会被列出"]
    pub fn set_part_number_marker_as_usize(self, value: usize) -> Self {
        self.insert("part-number_marker".into(), value.to_string().into())
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
impl ResponseBody {
    #[doc = "获取 在服务端申请的 Multipart Upload 任务 id"]
    pub fn get_upload_id_as_str(&self) -> &str {
        self.0
            .as_object()
            .unwrap()
            .get("uploadId")
            .unwrap()
            .as_str()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 在服务端申请的 Multipart Upload 任务 id"]
    pub fn set_upload_id_as_str(&mut self, new: String) -> Option<String> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("uploadId".to_owned(), new.into())
            .and_then(|val| match val {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
    }
}
impl ResponseBody {
    #[doc = "获取 UploadId 的过期时间 UNIX 时间戳，过期之后 UploadId 不可用"]
    pub fn get_expired_at_as_i64(&self) -> i64 {
        self.0
            .as_object()
            .unwrap()
            .get("expireAt")
            .unwrap()
            .as_i64()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 UploadId 的过期时间 UNIX 时间戳，过期之后 UploadId 不可用"]
    pub fn set_expired_at_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("expireAt".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl ResponseBody {
    #[doc = "获取 UploadId 的过期时间 UNIX 时间戳，过期之后 UploadId 不可用"]
    pub fn get_expired_at_as_u64(&self) -> u64 {
        self.0
            .as_object()
            .unwrap()
            .get("expireAt")
            .unwrap()
            .as_u64()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 UploadId 的过期时间 UNIX 时间戳，过期之后 UploadId 不可用"]
    pub fn set_expired_at_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("expireAt".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
impl ResponseBody {
    #[doc = "获取 下次继续列举的起始位置，0 表示列举结束，没有更多分片"]
    pub fn get_part_number_marker_as_i64(&self) -> i64 {
        self.0
            .as_object()
            .unwrap()
            .get("partNumberMarker")
            .unwrap()
            .as_i64()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 下次继续列举的起始位置，0 表示列举结束，没有更多分片"]
    pub fn set_part_number_marker_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("partNumberMarker".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl ResponseBody {
    #[doc = "获取 下次继续列举的起始位置，0 表示列举结束，没有更多分片"]
    pub fn get_part_number_marker_as_u64(&self) -> u64 {
        self.0
            .as_object()
            .unwrap()
            .get("partNumberMarker")
            .unwrap()
            .as_u64()
            .unwrap()
    }
}
impl ResponseBody {
    #[doc = "设置 下次继续列举的起始位置，0 表示列举结束，没有更多分片"]
    pub fn set_part_number_marker_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("partNumberMarker".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "所有已经上传的分片信息"]
pub struct ListedParts(serde_json::Value);
impl ListedParts {
    #[allow(dead_code)]
    pub(crate) fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}
impl From<ListedParts> for serde_json::Value {
    #[inline]
    fn from(val: ListedParts) -> Self {
        val.0
    }
}
impl std::convert::AsRef<serde_json::Value> for ListedParts {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl std::convert::AsMut<serde_json::Value> for ListedParts {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.0
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "单个已经上传的分片信息"]
pub struct ListedPartInfo(serde_json::Value);
impl ListedPartInfo {
    #[allow(dead_code)]
    pub(crate) fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}
impl From<ListedPartInfo> for serde_json::Value {
    #[inline]
    fn from(val: ListedPartInfo) -> Self {
        val.0
    }
}
impl std::convert::AsRef<serde_json::Value> for ListedPartInfo {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl std::convert::AsMut<serde_json::Value> for ListedPartInfo {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.0
    }
}
impl ListedPartInfo {
    #[doc = "获取 分片大小"]
    pub fn get_size_as_i64(&self) -> i64 {
        self.0
            .as_object()
            .unwrap()
            .get("size")
            .unwrap()
            .as_i64()
            .unwrap()
    }
}
impl ListedPartInfo {
    #[doc = "设置 分片大小"]
    pub fn set_size_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("size".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl ListedPartInfo {
    #[doc = "获取 分片大小"]
    pub fn get_size_as_u64(&self) -> u64 {
        self.0
            .as_object()
            .unwrap()
            .get("size")
            .unwrap()
            .as_u64()
            .unwrap()
    }
}
impl ListedPartInfo {
    #[doc = "设置 分片大小"]
    pub fn set_size_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("size".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
impl ListedPartInfo {
    #[doc = "获取 分片内容的 etag"]
    pub fn get_etag_as_str(&self) -> &str {
        self.0
            .as_object()
            .unwrap()
            .get("etag")
            .unwrap()
            .as_str()
            .unwrap()
    }
}
impl ListedPartInfo {
    #[doc = "设置 分片内容的 etag"]
    pub fn set_etag_as_str(&mut self, new: String) -> Option<String> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("etag".to_owned(), new.into())
            .and_then(|val| match val {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
    }
}
impl ListedPartInfo {
    #[doc = "获取 每一个上传的分片都有一个标识它的号码"]
    pub fn get_part_number_as_i64(&self) -> i64 {
        self.0
            .as_object()
            .unwrap()
            .get("partNumber")
            .unwrap()
            .as_i64()
            .unwrap()
    }
}
impl ListedPartInfo {
    #[doc = "设置 每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("partNumber".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl ListedPartInfo {
    #[doc = "获取 每一个上传的分片都有一个标识它的号码"]
    pub fn get_part_number_as_u64(&self) -> u64 {
        self.0
            .as_object()
            .unwrap()
            .get("partNumber")
            .unwrap()
            .as_u64()
            .unwrap()
    }
}
impl ListedPartInfo {
    #[doc = "设置 每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("partNumber".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
impl ListedPartInfo {
    #[doc = "获取 分片上传时间 UNIX 时间戳"]
    pub fn get_put_time_as_i64(&self) -> i64 {
        self.0
            .as_object()
            .unwrap()
            .get("put_time")
            .unwrap()
            .as_i64()
            .unwrap()
    }
}
impl ListedPartInfo {
    #[doc = "设置 分片上传时间 UNIX 时间戳"]
    pub fn set_put_time_as_i64(&mut self, new: i64) -> Option<i64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("put_time".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl ListedPartInfo {
    #[doc = "获取 分片上传时间 UNIX 时间戳"]
    pub fn get_put_time_as_u64(&self) -> u64 {
        self.0
            .as_object()
            .unwrap()
            .get("put_time")
            .unwrap()
            .as_u64()
            .unwrap()
    }
}
impl ListedPartInfo {
    #[doc = "设置 分片上传时间 UNIX 时间戳"]
    pub fn set_put_time_as_u64(&mut self, new: u64) -> Option<u64> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("put_time".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
impl ListedParts {
    #[doc = "解析 JSON 得到 ListedPartInfo 列表"]
    pub fn to_listed_part_info_vec(&self) -> Vec<ListedPartInfo> {
        self.0
            .as_array()
            .unwrap()
            .iter()
            .cloned()
            .map(ListedPartInfo::new)
            .collect()
    }
}
impl From<Vec<ListedPartInfo>> for ListedParts {
    #[inline]
    fn from(val: Vec<ListedPartInfo>) -> Self {
        Self(serde_json::Value::from(val))
    }
}
impl ListedParts {
    pub fn len(&self) -> usize {
        self.0.as_array().unwrap().len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.as_array().unwrap().is_empty()
    }
}
impl ListedParts {
    #[doc = "在列表的指定位置插入 JSON ListedPartInfo"]
    pub fn insert_listed_part_info(&mut self, index: usize, val: ListedPartInfo) {
        self.0.as_array_mut().unwrap().insert(index, val.into());
    }
}
impl ListedParts {
    #[doc = "在列表的指定位置移出 JSON ListedPartInfo"]
    pub fn remove_as_listed_part_info(&mut self, index: usize) -> ListedPartInfo {
        ListedPartInfo::new(self.0.as_array_mut().unwrap().remove(index))
    }
}
impl ListedParts {
    #[doc = "在列表尾部追加 JSON ListedPartInfo"]
    pub fn push_listed_part_info(&mut self, val: ListedPartInfo) {
        self.0.as_array_mut().unwrap().push(val.into());
    }
}
impl ListedParts {
    #[doc = "在列表尾部取出 JSON ListedPartInfo"]
    pub fn pop_listed_part_info(&mut self) -> Option<ListedPartInfo> {
        self.0
            .as_array_mut()
            .unwrap()
            .pop()
            .map(ListedPartInfo::new)
    }
}
impl ResponseBody {
    #[doc = "获取 返回所有已经上传成功的分片信息"]
    pub fn get_parts(&self) -> ListedParts {
        ListedParts::new(self.0.as_object().unwrap().get("parts").cloned().unwrap())
    }
}
impl ResponseBody {
    #[doc = "设置 返回所有已经上传成功的分片信息"]
    pub fn set_parts(&mut self, new: ListedParts) -> Option<ListedParts> {
        self.0
            .as_object_mut()
            .unwrap()
            .insert("parts".to_owned(), new.into())
            .map(ListedParts::new)
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
        upload_token: impl qiniu_http_client::upload_token::UploadTokenProvider
            + std::clone::Clone
            + 'static,
    ) -> SyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self
                .0
                .get(&[qiniu_http_client::ServiceName::Up], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::uptoken(upload_token));
            builder.idempotent(qiniu_http_client::Idempotent::Default);
            builder.path(crate::base_utils::join_path(
                "/buckets",
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
        upload_token: impl qiniu_http_client::upload_token::UploadTokenProvider
            + std::clone::Clone
            + 'static,
    ) -> AsyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self
                .0
                .async_get(&[qiniu_http_client::ServiceName::Up], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::uptoken(upload_token));
            builder.idempotent(qiniu_http_client::Idempotent::Default);
            builder.path(crate::base_utils::join_path(
                "/buckets",
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
