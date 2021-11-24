//! Node native addon
//!
//! Provides an abstraction of C bindings to use our libraries directly from Typescript.<br>
//! When the abstraction is used, there are these advantages:
//! - Faster
//! - Safer
//!
//! To achieve this abstraction, we use the NAPI crate:
//! - [`NAPI Crate`]
//! - [`NAPI Doc`]
//! - [`NAPI GitHub`]
//! - [`NAPI Examples`]
//!
//! [`NAPI Crate`]: https://crates.io/crates/napi/2.0.0-alpha.4
//! [`NAPI Doc`]: https://docs.rs/napi/2.0.0-alpha.4/napi/
//! [`NAPI GitHub`]: https://github.com/napi-rs/napi-rs
//! [`NAPI Examples`]: https://github.com/napi-rs/napi-rs/tree/main/examples/napi
//!
//! Currently we have these libraries:
//! - uuid: generates uuids

#[macro_use]
extern crate napi_derive;

mod uuid;
