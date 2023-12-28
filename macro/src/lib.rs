use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::{For, Gt, Lt},
    Expr, ExprClosure, GenericParam, Ident, Pat, ReturnType, Token, Type,
};

/*
The `name!` macro makes it easier to work with named values by generating the boilerplate trait impls for `NameFn`.

Example:
fn foo() {
    name!(1, for<A> |a: Named<u32, A>| -> () {
        name!(2, for<B> |b: Named<u32, B>| -> () {
            do_stuff(a, b);
        })
    });
}

becomes:
fn foo() {
    struct Fn1 {}

    impl departed_core::named::NameFn for Fn1 {
        type In = u32;
        type Out = ();

        fn call<A>(self, a: departed_core::named::Named<Self::In, A>) -> Self::Out {
            let Fn1 {
            } = self;

            struct Fn2<A> {
                a: Named<u32, A>
            }

            impl<A> departed_core::named::NameFn for Fn2<A> {
                type In = u32;
                type Out = ();
                fn call<B>(self, b: departed_core::named::Named<Self::In, B>) -> Self::Out {
                    let Fn2 {
                        a
                    } = self;
                    do_stuff(a, b)
                }
            }

            departed_core::named::name(2, Fn2 {
                a
            })
        }
    }

    departed_core::named::name(1, Fn1 {})
}
*/

mod kw {
    syn::custom_keyword!(Named);
}

struct NameClosure {
    expr: Expr,
    ident: Ident,
    name: GenericParam,
    in_type: Type,
    out_type: Type,
    body: Expr,
}

impl Parse for NameClosure {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let expr: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        input.parse::<For>()?;
        input.parse::<Lt>()?;
        let name: GenericParam = input.parse()?;
        input.parse::<Gt>()?;
        input.parse::<Token![|]>()?;
        let ident: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        input.parse::<kw::Named>()?;
        input.parse::<Lt>()?;
        let in_type: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let attached_name: GenericParam = input.parse()?;
        input.parse::<Gt>()?;
        input.parse::<Token![|]>()?;
        input.parse::<Token![->]>()?;
        let out_type: Type = input.parse()?;
        let body: Expr = input.parse()?;

        assert!(name == attached_name);

        Ok(NameClosure {
            expr,
            ident,
            name,
            in_type,
            out_type,
            body,
        })
    }
}

#[proc_macro]
pub fn name(input: TokenStream) -> TokenStream {
    let NameClosure {
        expr,
        ident,
        name,
        in_type,
        out_type,
        body,
    } = parse_macro_input!(input as NameClosure);

    quote! {
        {
            struct NameClosure;

            impl departed_core::named::NameFn for NameClosure {
                type In = #in_type;
                type Out = #out_type;

                fn call<#name>(#ident: departed_core::named::Named<Self::In, #name>) -> Self::Out {
                    #body
                }
            }

            departed_core::named::name(#expr, NameClosure)
        }
    }
    .into()
}
