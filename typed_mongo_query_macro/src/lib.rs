extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field};

#[proc_macro_derive(Queriable, attributes(mongo_strict_query))]
pub fn queriable_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used to store the fields that will be included in the query struct
    let mut query_fields = Vec::new();
    let mut with_methods = Vec::new();

    // Extract the struct's name and fields
    let name = input.ident;
    let query_struct_name = syn::Ident::new(&format!("{}Query", name), name.span());
    let fields = if let syn::Data::Struct(data) = input.data {
        data.fields
    } else {
        // Handle non-structs, if necessary
        unimplemented!()
    };

    for field in fields {
        if let Some(ident) = &field.ident {
            let ty = &field.ty;
            query_fields.push(quote! { #ident: Option<Comparison<#ty>>, });

            let method_name = syn::Ident::new(&format!("with_{}", ident), ident.span());
            with_methods.push(quote! {
                pub fn #method_name(mut self, value: Comparison<#ty>) -> Self {
                    self.#ident = Some(value);
                    self
                }
            });
        }
    }

    // Generate the output tokens
    let expanded = quote! {
        struct #query_struct_name {
            #(#query_fields)*
        }

        impl #query_struct_name {
            #(#with_methods)*
        }

        // Implement `Parameter` and `Document` traits for `#query_struct_name`
        // ...
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
