use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn miden_test(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    let main_wrapper = quote! {
        pub use libtest_mimic as __libtest_mimic_miden_test;

        fn runner(
            test: fn() -> (),
        ) -> impl FnOnce() -> Result<(), __libtest_mimic_miden_test::Failed> + Send + 'static {
            move || {
                test();
                Ok(())
            }
        }

        fn fail() {
            println!("Cheese");
            panic!("Fail");
        }


        fn main() {
            println!("Hello");
            let args = __libtest_mimic_miden_test::Arguments::from_args();

            let tests = vec![__libtest_mimic_miden_test::Trial::test("fail", runner(fail)),
            ];
            let con = __libtest_mimic_miden_test::run(&args, tests);

            std::dbg!(&con);
            con.exit()
        }
    };

    TokenStream::from(main_wrapper)
}
