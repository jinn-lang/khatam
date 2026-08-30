//! `khatam-chain` owns chain following with rewinds, the provider capability traits, the typed
//! events, and the snapshots it emits.
//!
//! It does not own the era types it decodes (`khatam-era`), the emulator (`khatam-emulator`), the
//! rules (`khatam-rules`), or the store that consumes its events (a foreign component).
//!
//! Realization status: designed. This crate is a structural shell; it implies no implemented
//! behavior.

#![forbid(unsafe_code)]
