// Only intended to be used by the macro crate
pub struct MidenTest {
    pub name: &'static str,
    // pub test_fn: fn() -> Result<(), libtest_mimic::Failed>,
    pub test_fn: fn() -> (),
}

inventory::collect!(MidenTest);

pub use inventory::submit as miden_test_submit;

// // Wrapper around inventory submit
// #[macro_export]
// macro_rules! miden_test_submit {
//     ($($value:tt)*) => {
//         inventory::submit!( $(value)* )
//     };
// }

// Wrapper used to make normal rust function.
fn runner(test: fn() -> ()) -> impl FnOnce() -> Result<(), libtest_mimic::Failed> + Send + 'static {
    move || {
        test();
        Ok(())
    }
}

impl From<MidenTest> for libtest_mimic::Trial {
    fn from(value: MidenTest) -> Self {
        libtest_mimic::Trial::test(value.name, runner(value.test_fn))
    }
}

pub struct MidenTestArguments(libtest_mimic::Arguments);

impl From<MidenTestArguments> for libtest_mimic::Arguments {
    fn from(value: MidenTestArguments) -> Self {
        value.0
    }
}

impl MidenTestArguments {
    pub fn from_args() -> Self {
        let inner_args = libtest_mimic::Arguments::from_args();
        Self(inner_args)
    }
}

pub fn run(args: MidenTestArguments, tests: Vec<MidenTest>) {
    let args = args.into();
    let tests = tests.into_iter().map(|t| t.into()).collect();

    let conclusion = libtest_mimic::run(&args, tests);

    conclusion.exit()
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
