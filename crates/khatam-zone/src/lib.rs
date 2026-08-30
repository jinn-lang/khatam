//! `khatam-zone` owns the zone shape and its all-or-nothing application.
//!
//! It does not own the rules it applies (`khatam-rules`), the era types beneath them
//! (`khatam-era`), the emulator (`khatam-emulator`), or the zone plan of any foreign component.
//!
//! Realization status: designed. This crate is a structural shell; it implies no implemented
//! behavior.

#![forbid(unsafe_code)]
