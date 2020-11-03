mod client;
mod region;
mod resolver;

pub(super) use client::{
    make_dumb_client_builder, make_error_response_client_builder,
    make_fixed_response_client_builder,
};
pub(super) use region::{chaotic_up_domains_region, single_up_domain_region};
pub(super) use resolver::{make_dumb_resolver, make_error_resolver, make_static_resolver};