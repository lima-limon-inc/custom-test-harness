#![no_std]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[cfg(feature = "test-flag")]
fn is_test() -> bool {
    true
}

#[cfg(not(feature = "test-flag"))]
fn is_test() -> bool {
    // true // Used when debugging.
    false
}

fn load_account(function: &mut syn::ItemFn) {
    let mut found_packages_vars = Vec::new();

    for arg in function.sig.inputs.iter() {
        let syn::FnArg::Typed(arg) = arg else {
            continue;
        };
        let syn::Type::Path(syn::TypePath { path, .. }) = *arg.ty.clone() else {
            continue;
        };
        // The last token in the segments vector is the actual type, the rest
        // are just path specifiers.
        let Some(maybe_package) = path.segments.last() else {
            continue;
        };

        if maybe_package.ident != "Package" {
            continue;
        }

        let syn::Pat::Ident(package_var_binding) = arg.pat.as_ref() else {
            panic!("Couldn't find binding for package")
        };
        found_packages_vars.push(package_var_binding.ident.clone());
    }

    let var_name = found_packages_vars[0].clone();

    let load_package: Vec<syn::Stmt> = syn::parse_quote! {
        let path = "/Users/fabri/Repositories/counter-contract_original/counter-contract/target/miden/debug/counter_contract.masp";
        let bytes = std::fs::read(path).unwrap();
        let #var_name = miden_mast_package::Package::read_from_bytes(&bytes).unwrap();
    };

    // We add the the lines required to load the generated Package.
    for (i, package) in load_package.iter().enumerate() {
        function.block.as_mut().stmts.insert(i, package.clone());
    }
}

#[proc_macro_attribute]
pub fn miden_test(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);

    let fn_ident = input_fn.sig.ident.clone();
    let fn_name = fn_ident.clone().span().source_text().unwrap();

    load_account(&mut input_fn);

    input_fn.sig.inputs.clear();

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
