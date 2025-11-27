use midenc_harness_macros::miden_test;

#[miden_test]
fn jamon() {
    assert_eq!(1, 2)
}

#[miden_test]
fn queso() {
    assert_eq!(1, 1)
}
