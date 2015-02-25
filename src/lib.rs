#![feature(core, collections, old_io, std_misc, old_path)]
#![cfg_attr(test, feature(io))]

extern crate libc;
extern crate url;
#[cfg(test)] #[macro_use] extern crate log;

extern crate "curl-sys" as curl_ffi;
#[cfg(unix)] extern crate "openssl-sys" as openssl;

pub use ffi::easy::ProgressCb;
pub use ffi::err::ErrCode;

// Version accessors
pub use ffi::version::{
    Version,
    version,
    version_info,
    Protocols
};

mod ffi;
pub mod http;

#[cfg(test)]
mod test;
