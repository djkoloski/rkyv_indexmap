//! # rkyv_indexmap
//!
//! Implementations of `IndexMap` and `IndexSet` that are compatible with rkyv.

#![deny(broken_intra_doc_links)]
#![deny(missing_docs)]
#![deny(missing_crate_level_docs)]
#![no_std]

extern crate alloc;

pub mod index_map;
pub mod index_set;

pub use index_map::ArchivedIndexMap;
pub use index_set::ArchivedIndexSet;
