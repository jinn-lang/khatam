//! `khatam` is the facade crate of this workspace. It owns the public surface of the ledger
//! adapter: it re-exports every crate it depends on under a short name. Other components consume
//! this workspace through this crate alone.
//!
//! It does not own any type, rule or codec of its own; every item it exposes belongs to the crate
//! that declares it, and the one-owner assignment of this workspace is unchanged by the re-export.
//!
//! Realization status: designed. This crate is a structural shell; it implies no implemented
//! behavior.

#![forbid(unsafe_code)]

pub use khatam_build as build;
pub use khatam_chain as chain;
pub use khatam_emulator as emulator;
pub use khatam_era as era;
pub use khatam_rules as rules;
pub use khatam_sign as sign;
pub use khatam_zone as zone;
