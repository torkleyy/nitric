//! # `nitric-component`
//!
//! This crate implements component storages, providing a mapping from IDs / Entities to data
//! points.
//!
//! ## Traits
//!
//! ### Split traits
//!
//! Often you might expect traits like `Storage`. However, there is no such trait. This is simply
//! because different storages have different traits, so the different properties are split into
//! multiple traits. That means all code will only depend on the traits it actually uses.
//!
//! Traits are always grouped together in modules so you have an overview of what storages might be
//! able to do.
//!
//! ## Structure
//!
//! This crate is split into a generic interface and implementations of these interfaces.
//!
//! Generic interfaces are in
//!
//! * `allocator`
//! * `id`
//! * `storage`
//!
//! Implementations are in
//!
//! * `impls`
//!

#[macro_use]
extern crate err_derive;

pub mod allocator;
pub mod id;
pub mod storage;

pub mod error;
pub mod impls;
