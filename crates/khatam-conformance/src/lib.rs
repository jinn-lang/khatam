//! `khatam-conformance` owns the terminal quarantine of this workspace: its vectors, fixtures and
//! adapters. It is terminal: it depends on the crates of this workspace and nothing in this
//! workspace depends on it.
//!
//! It does not own the era types, the rules, the zone shape, chain following, the emulator,
//! completion or signing that it exercises. Adapters to foreign components are wired by the
//! portfolio, never by a path dependency inside this workspace.
//!
//! Realization status: designed. This crate is a structural shell; it implies no implemented
//! behavior.

#![forbid(unsafe_code)]
