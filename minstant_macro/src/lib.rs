// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn timing(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let handle_result = syn::parse_macro_input!(args as syn::ExprClosure);

    let syn::ItemFn {
        attrs,
        vis,
        block,
        sig,
    } = input;

    let syn::Signature {
        output: return_type,
        inputs: params,
        unsafety,
        asyncness,
        constness,
        abi,
        ident,
        generics:
            syn::Generics {
                params: gen_params,
                where_clause,
                ..
            },
        ..
    } = sig;

    quote::quote!(
        #(#attrs) *
        #vis #constness #unsafety #asyncness #abi fn #ident<#gen_params>(#params) #return_type
        #where_clause
        {
            let __start = minstant::instant::Instant::now();
            let __res = (|| { #block })();
            let __end = minstant::instant::Instant::now();
            let __elapsed_cycles =  __end - __start;
            let _: () = (#handle_result)(__elapsed_cycles);
            __res
        }
    )
    .into()
}
