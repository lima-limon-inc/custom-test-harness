use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn miden_test(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let main_wrapper = quote! {
        fn main() {
            println!("Cheese");
        }
    };

    TokenStream::from(main_wrapper)
}
