#![no_std]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn miden_test(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name_str = stringify!(input_fn.sig.ident.clone());
    let fn_name = input_fn.sig.ident.clone();


    let function = quote! {

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

#[proc_macro_attribute]
pub fn miden_tests(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_module = parse_macro_input!(item as syn::ItemMod);

    let module = quote! {
        use miden_harness_lib;


        #[cfg(test)]
        #input_module

        fn main() {
            let args = miden_harness_lib::MidenTestArguments::from_args();

            miden_harness_lib::run(args);
        }
    };

    module.into()
}
