#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    non_ascii_idents,
    indirect_structural_match,
    trivial_casts,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications
)]

mod file_type;
mod upload_policy;
mod upload_token;
pub use file_type::{FileType, InvalidFileType};
pub use serde_json;
pub use upload_policy::{UploadPolicy, UploadPolicyBuilder};
use upload_token::FromUploadPolicy;
pub use upload_token::{
    CachedUploadTokenProvider, ObjectUploadTokenProvider, ParseError, ParseResult,
    StaticUploadTokenProvider, UploadTokenProvider,
};

pub mod preclude {
    pub use super::UploadTokenProvider;
}
