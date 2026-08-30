//! `khatam-era` owns the era body, witness, output, script and parameter types under the pinned
//! CDDL, their byte-exact codecs, and `BodyId`.
//!
//! It does not own rule evaluation (`khatam-rules`), the zone shape (`khatam-zone`), chain
//! following (`khatam-chain`), completion (`khatam-build`), signing (`khatam-sign`), or the
//! neutral plan record of any foreign component.
//!
//! Realization status: designed. This crate is a structural shell; it implies no implemented
//! behavior.

#![forbid(unsafe_code)]
