use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

static mut PROCESSED: bool = false;

#[proc_macro_attribute]
pub fn miden_test(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name_str = input_fn.sig.ident.to_string();
    let fn_name = input_fn.sig.ident.clone();

    // We use PROCESSED in order to recreate C's #ifndef, and thus only generate a single main function.
    let prelude = if unsafe { PROCESSED } {
        quote! {}
    } else {
        // After including the PRELUDE once, we never include it again.
        unsafe {
            PROCESSED = true;
        };
        quote! {
            pub use miden_harness_lib as __miden_harness_lib;

            fn main() {
                let args = __miden_harness_lib::MidenTestArguments::from_args();

                __miden_harness_lib::run(args);
            }

        }
    };

    let function = quote! {
        #prelude

        __miden_harness_lib::miden_test_submit!(
            __miden_harness_lib::MidenTest {
                name: #fn_name_str,
                test_fn: #fn_name,
            }
        );

        #[cfg(test)]
        #input_fn
    };

    TokenStream::from(function)
}
