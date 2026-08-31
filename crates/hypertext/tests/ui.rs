//! Compile-fail and pass-case coverage for macro diagnostics.
#![cfg(feature = "alloc")]
#![allow(missing_docs)]
#![allow(unexpected_cfgs)]

#[cfg(not(miri))]
#[test]
fn macro_diagnostics() {
    let tests = trybuild::TestCases::new();
    tests.pass("tests/ui/pass/*.rs");
    tests.compile_fail("tests/ui/fail/*.rs");
}
