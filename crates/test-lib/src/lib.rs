fn hello() {}

// Only intended to be used by the macro crate
pub struct MidenTest {
    pub name: &'static str,
    pub test_fn: fn() -> Result<(), libtest_mimic::Failed>,
}

inventory::collect!(MidenTest);

// fn main() {
//     println!("Hello")
// }

// mod test {
// use midenc_harness_macros::miden_test;

// #[miden_test]
// fn jamon() {
//     1 + 1;
// }

// fn main() {}
// }

// #[miden_test]
// pub fn ham() {
//     println!("Ham")
// }
// #[test]
// fn bar() {
//     panic!("")
// }
// }

// fn main() {
//     crate::test::main()
// }
