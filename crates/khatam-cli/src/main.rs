//! `khatam-cli` owns the operator surface over the `khatam` facade: the command names, their
//! arguments and their output shapes.
//!
//! It does not own the era types, the rules, the zone shape, chain following, the emulator,
//! completion or signing; every operation it names belongs to the facade and, through it, to the
//! crate that declares it.
//!
//! Realization status: designed. This crate is a structural shell; it implies no implemented
//! behavior.

#![forbid(unsafe_code)]

fn main() {}
