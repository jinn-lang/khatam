//! `khatam-rules` owns the phase-one rule families as pure functions, one definition each,
//! including the zone rules: L4 zone balance, L7 and L16 observer forcing, L10 atomicity, L11
//! sub-shape, L12 pre-existing inputs, L14 two levels, and L15 shared witnesses.
//!
//! It does not own the era types it reads (`khatam-era`), the zone shape that applies a rule set
//! all-or-nothing (`khatam-zone`), the ledger model of any foreign component, or evaluation of a
//! script.
//!
//! Realization status: designed. This crate is a structural shell; it implies no implemented
//! behavior.

#![forbid(unsafe_code)]
