use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn names(_args: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(item as ItemFn);

    quote! {
        /*
        #(
            struct NameCtn1;
            impl departed_core::named::NameFn for NameCtn1 {
                type In = /* generate this */;
                type Out = /* generate this */;

                fn call<N>(named: Named<Self::In, N>) -> Self::Out {
                    //... use named
                }
            }
        )*
        */
    }
    .into()
}
