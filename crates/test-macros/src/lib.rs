use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Item, ItemFn};

static mut PROCESSED: bool = false;

struct Test {
    pub name: &'static str,
    pub test_fn: fn() -> Result<(), libtest_mimic::Failed>,
}

inventory::collect!(Test);

#[proc_macro_attribute]
pub fn miden_test(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");

    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name_str = input_fn.sig.ident.to_string();
    let fn_name = input_fn.sig.ident.clone();

    // We use PROCESSED in order to recreate C's #ifndef
    let prelude = if unsafe { PROCESSED } {
        quote! {}
    } else {
        // After including the PRELUDE once, we never include it again.
        unsafe {
            PROCESSED = true;
        };
        quote! {
            pub use inventory as __inventory_miden_test;
            pub use libtest_mimic as __libtest_mimic_miden_test;

            fn runner(
                test: fn() -> (),
            ) -> impl FnOnce() -> Result<(), __libtest_mimic_miden_test::Failed> + Send + 'static {
                move || {
                    test();
                    Ok(())
                }
            }

            fn main() {
                let args = __libtest_mimic_miden_test::Arguments::from_args();

                let tests = vec![__libtest_mimic_miden_test::Trial::test(#fn_name_str, runner(#fn_name)),
                ];
                let con = __libtest_mimic_miden_test::run(&args, tests);

                con.exit()
            }

        }
    };

    // The prelude is only added once.
    let function = quote! {
        #prelude

        // __inventory_miden_test::submit!(Test {
        //     name: #fn_name_str.as_str(),
        //     test_fn: #fn_name,
        // });

        #input_fn
    };

    TokenStream::from(function)
}
