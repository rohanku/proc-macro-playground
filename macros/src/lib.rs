extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn my_attr_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: proc_macro2::TokenStream = item.into();
    quote! {
        struct AutoStruct {
            auto_field: u32,
        }
        #item
    }
    .into()
}
