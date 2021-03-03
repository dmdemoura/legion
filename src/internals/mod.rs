pub mod cons;
pub mod entity;
pub mod entry;
pub mod event;
pub mod hash;
pub mod insert;
pub mod iter;
pub mod permissions;
pub mod query;
#[cfg(feature = "serialize")]
pub mod serialize;
pub mod storage;
pub mod subworld;
pub mod systems;
pub mod world;

mod alloc_prelude {
    pub use alloc::{
        borrow::ToOwned,
        boxed::Box,
        format,
        string::{String, ToString},
        vec,
        vec::Vec,
    };
}

#[cfg(feature = "std")]
mod sync {
    pub use parking_lot::*;
}

#[cfg(feature = "adven-gba")]
mod sync {
    pub use adven::sync::*;
}

#[cfg(feature = "std")]
mod hashmap {
    pub use std::collections::{
        hash_map::{self, *},
        hash_set::{self, *},
        HashMap, HashSet,
    };
}

#[cfg(feature = "alloc")]
mod hashmap {
    pub use hashbrown::*;
}

#[cfg(not(feature = "adven"))]
mod id {
    pub type ID = u64;
    pub type NonZeroID = core::num::NonZeroU64;
    pub type AtomicID = core::sync::atomic::AtomicU64;
    pub type IDHasher = super::hash::U64Hasher;
}

#[cfg(feature = "adven")]
mod id {
    pub type ID = u32;
    pub type NonZeroID = core::num::NonZeroU32;
    pub type AtomicID = core::sync::atomic::AtomicU32;
    pub type IDHasher = super::hash::U32Hasher;
}
