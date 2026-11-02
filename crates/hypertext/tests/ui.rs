//! Compile-fail and pass-case coverage for macro diagnostics.
#![cfg(feature = "alloc")]

#[cfg(not(miri))]
#[test]
fn macro_diagnostics() {
    let tests = trybuild::TestCases::new();
    tests.pass("tests/ui/pass/*.rs");
    tests.compile_fail("tests/ui/fail/*.rs");
}
