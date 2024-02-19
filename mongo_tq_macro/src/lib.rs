extern crate proc_macro;
extern crate syn;

use mongo_tq::query;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Field};

#[proc_macro_derive(Queryable, attributes(mongo_tq))]
pub fn queryable_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used to store the fields that will be included in the query struct
    let mut query_fields = Vec::new();
    let mut with_methods = Vec::new();
    let mut field_initializers = Vec::new();

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
            let mongo_tq_attr = field
                .attrs
                .iter()
                .find(|attr| attr.path.is_ident("mongo_tq"));
            if mongo_tq_attr.is_none() {
                continue;
            }
            let ty = &field.ty;
            query_fields.push(quote! { #ident: Option<Comparison<#ty>>, });
            field_initializers.push(quote! { #ident: None });

            let method_name = syn::Ident::new(&format!("with_{}", ident), ident.span());
            with_methods.push(quote! {
                pub fn #method_name(mut self, value: Comparison<#ty>) -> Self {
                    self.#ident = Some(value);
                    self
                }
            });
        }
    }

    // Generate the `all` method
    let gen_all_method = quote! {
            pub fn all() -> Self {
                Self {
                    #(#field_initializers),*
                }
            }
    };

    with_methods.push(gen_all_method);

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
