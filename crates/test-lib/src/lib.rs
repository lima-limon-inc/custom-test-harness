fn hello() {}

struct Test {
    pub name: &'static str,
    pub test_fn: fn() -> Result<(), libtest_mimic::Failed>,
}

#[macro_export]
macro_rules! inv_collect {
    ($ty:ty) => {
        inventory::collect!($ty);
    };
}

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
