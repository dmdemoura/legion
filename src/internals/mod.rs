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
mod hashmap {
    pub use std::collections::hash_map::{self, *};
    pub use std::collections::hash_set::{self, *};
    pub use std::collections::HashMap;
    pub use std::collections::HashSet;
}

#[cfg(feature = "alloc")]
mod hashmap {
    pub use hashbrown::*;
}
