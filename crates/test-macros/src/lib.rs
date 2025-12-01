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

    let fn_ident = input_fn.sig.ident.clone();
    let fn_name = fn_ident.clone().span().source_text().unwrap();

    let function = quote! {
        miden_harness_lib::miden_test_submit!(
            miden_harness_lib::MidenTest {
                name: #fn_name,
                test_fn: #fn_ident,
            }
        );

        #[cfg(test)]
        #input_fn
    };

    TokenStream::from(function)
}

#[proc_macro_attribute]
pub fn miden_test_block(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input_module = parse_macro_input!(item as syn::ItemMod);

    // We add an internal "use" here in order for the tests inside the `mod tests`
    // block to use the `miden_test` macro without needing to pass the full path.
    let internal_use = syn::parse_quote! {
        use miden_harness_macros::miden_test;
    };
    input_module
        .content
        .as_mut()
        .unwrap()
        .1
        .insert(0, internal_use);

    #[cfg(feature = "test-flag")]
    fn is_test() -> bool {
        true
    }

    #[cfg(not(feature = "test-flag"))]
    fn is_test() -> bool {
        // true // Used when debugging.
        false
    }

    let module = if is_test() {
        quote! {
            #input_module
        }
    } else {
        quote! {}
    };

    let main_function = if is_test() {
        quote! {
            use miden_harness_lib;
            fn main() {
                let args = miden_harness_lib::MidenTestArguments::from_args();

                miden_harness_lib::run(args);
            }
        }
    } else {
        quote! {}
    };

    let block = quote! {
        #module

        #main_function
    };

    block.into()
}
