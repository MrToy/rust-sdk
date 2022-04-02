#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![deny(
    single_use_lifetimes,
    missing_debug_implementations,
    large_assignments,
    exported_private_dependencies,
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_docs,
    non_ascii_idents,
    indirect_structural_match,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

//! # qiniu-objects-manager
//!
//! ## 七牛对象管理
//!
//! 基于 `qiniu-apis` 提供针对七牛对象的管理功能
//! （同时提供阻塞客户端和异步客户端，异步客户端则需要启用 `async` 功能），
//! 主要负责七牛对象的列举和操作。
//!
//! ### 对象元信息获取
//!
//! ```
//! use qiniu_objects_manager::{apis::credential::Credential, ObjectsManager};
//!
//! # async fn example() -> anyhow::Result<()> {
//! let credential = Credential::new("abcdefghklmnopq", "1234567890");
//! let object_manager = ObjectsManager::builder(credential).build();
//! let bucket = object_manager.bucket("test-bucket");
//!
//! let response = bucket.stat_object("test-key").async_call().await?;
//! let object = response.into_body();
//! println!("fsize: {}", object.get_size_as_u64());
//! println!("hash: {}", object.get_hash_as_str());
//! println!("mime_type: {}", object.get_mime_type_as_str());
//! # Ok(())
//! # }
//! ```
//!
//! ### 对象批量元信息获取
//!
//! ```
//! use qiniu_objects_manager::{apis::credential::Credential, ObjectsManager, OperationProvider};
//! use futures::stream::TryStreamExt;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let credential = Credential::new("abcdefghklmnopq", "1234567890");
//! let object_manager = ObjectsManager::builder(credential).build();
//! let bucket = object_manager.bucket("test-bucket");
//! let mut ops = bucket.batch_ops();
//! ops.add_operation(bucket.stat_object("test-file-1"));
//! ops.add_operation(bucket.stat_object("test-file-2"));
//! ops.add_operation(bucket.stat_object("test-file-3"));
//! ops.add_operation(bucket.stat_object("test-file-4"));
//! ops.add_operation(bucket.stat_object("test-file-5"));
//! let mut stream = ops.async_call();
//! while let Some(object) = stream.try_next().await? {
//!     println!("fsize: {:?}", object.get_size_as_u64());
//!     println!("hash: {:?}", object.get_hash_as_str());
//!     println!("mime_type: {:?}", object.get_mime_type_as_str());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### 对象列举
//!
//! ```
//! use qiniu_objects_manager::{apis::credential::Credential, ObjectsManager};
//! use futures::stream::TryStreamExt;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let credential = Credential::new("abcdefghklmnopq", "1234567890");
//! let object_manager = ObjectsManager::builder(credential).build();
//! let bucket = object_manager.bucket("test-bucket");
//! let mut stream = bucket.list().stream();
//! while let Some(object) = stream.try_next().await? {
//!     println!("fsize: {:?}", object.get_size_as_u64());
//!     println!("hash: {:?}", object.get_hash_as_str());
//!     println!("mime_type: {:?}", object.get_mime_type_as_str());
//! }
//! # Ok(())
//! # }
//! ```

mod batch_operations;
mod bucket;
mod callbacks;
mod list;
mod objects_manager;
mod operation;

pub use batch_operations::{BatchOperations, BatchOperationsIterator, BatchSizeProvider};
pub use bucket::{Bucket, ListBuilder};
pub use list::{ListIter, ListVersion};
pub use objects_manager::{ObjectsManager, ObjectsManagerBuilder};
pub use operation::{
    AfterDays, CopyObjectBuilder, DeleteObjectBuilder, ModifyObjectLifeCycleBuilder, ModifyObjectMetadataBuilder,
    ModifyObjectStatusBuilder, MoveObjectBuilder, OperationProvider, SetObjectTypeBuilder, StatObjectBuilder,
    UnfreezeObjectBuilder,
};
pub use qiniu_apis as apis;

#[cfg(feature = "async")]
pub use {batch_operations::BatchOperationsStream, list::ListStream};

/// 将所有 Trait 全部重新导出，方便统一导入
pub mod prelude {
    pub use super::apis::http_client::prelude::*;
    pub use super::{BatchSizeProvider, OperationProvider};
}
