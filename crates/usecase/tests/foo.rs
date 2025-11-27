use miden_harness_macros::miden_test;

#[miden_test]
fn ham() {
    assert_eq!(1, 2)
}

#[miden_test]
fn cheese() {
    assert_eq!(1, 1)
}

#[test]
fn lettuce() {
    println!("This test is never run despite marked as #[test]. This is because we are using our own custom test harness.");
    assert_eq!(1, 3)
}
