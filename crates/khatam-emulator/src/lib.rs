//! `khatam-emulator` owns the pure in-memory ledger, forking, and deterministic replay.
//!
//! It does not own chain following against a live provider (`khatam-chain`), the rules it applies
//! (`khatam-rules`), the zone shape (`khatam-zone`), the era types (`khatam-era`), or the ledger
//! model of any foreign component.
//!
//! Realization status: designed. This crate is a structural shell; it implies no implemented
//! behavior.

#![forbid(unsafe_code)]
