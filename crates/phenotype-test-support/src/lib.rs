//! `phenotype-test-support`
//!
//! Shared BDD (cucumber) test harness, world state, and helper types for the
//! Phenotype ecosystem. Consumers add this crate to `[dev-dependencies]` and
//! can `use phenotype_test_support::bdd::*;` to obtain the previously
//! duplicated BDD step bodies, world state, and helper types.

pub mod bdd;
