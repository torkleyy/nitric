#![allow(clippy::match_bool)]
#![warn(missing_docs)]
#![deny(unused_must_use)]

//! # `nitric-component`
//!
//! This crate implements component storages, providing a mapping from IDs /
//! Entities to data points.
//!
//! ## Traits
//!
//! ### Split traits
//!
//! Often you might expect more methods in traits like `Storage`. However, many
//! properties of storages, allocators and IDs are optional and not implemented
//! for all instances. That means all code will only depend on the traits it
//! actually uses and is therefore reusable in many situations.
//!
//! Traits are always grouped together in modules so you have an overview of the
//! methods you can use.
//!
//! ## Structure
//!
//! This crate is split into a generic interface and implementations of these
//! interfaces.
//!
//! Generic interfaces are in
//!
//! * `allocator`
//! * `bit_set`
//! * `id`
//! * `storage`
//!
//! Implementations are in
//!
//! * `impls`
//!
//! Additionally, error types can be found in `error`.
//! Utility types can be found in `util`.
//! A prelude for common traits & types can be imported using `use
//! nitric_component::prelude::*`.

#[macro_use]
extern crate err_derive;

pub mod allocator;
pub mod bit_set;
pub mod id;
pub mod storage;

pub mod error;
pub mod impls;

pub mod prelude;
pub mod util;
