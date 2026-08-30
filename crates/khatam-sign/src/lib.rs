//! `khatam-sign` owns signing in the CARDANO_BODY domain and submission.
//!
//! It does not own body construction or hole filling (`khatam-build`), the era types it signs over
//! (`khatam-era`), the audit of signed bytes, or the custody of any key (foreign concerns).
//!
//! Realization status: designed. This crate is a structural shell; it implies no implemented
//! behavior.

#![forbid(unsafe_code)]
