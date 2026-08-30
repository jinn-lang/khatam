//! `khatam-build` owns `CompletionHole` filling only: funding, fee, collateral, min-ADA, change and
//! indices; the cost fixpoint taken across the evaluator trait; and reservation.
//!
//! It does not own any `ProofHole` or `Boundary`, the neutral plan record that states the holes, or
//! the evaluator behind the trait (foreign components); nor signing (`khatam-sign`) or the rules
//! it satisfies (`khatam-rules`).
//!
//! Realization status: designed. This crate is a structural shell; it implies no implemented
//! behavior.

#![forbid(unsafe_code)]
