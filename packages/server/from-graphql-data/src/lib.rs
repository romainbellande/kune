use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(FromGraphqlData)]
pub fn from_graphql_data(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Ensure the derived type is a struct.
    let data_struct = match input.data {
        Data::Struct(ref s) => s,
        _ => panic!("Only structs are supported"),
    };

    // Ensure the struct has named fields.
    let fields = match &data_struct.fields {
        Fields::Named(ref fields) => fields.named.clone(),
        _ => panic!("Only named fields are supported"),
    };

    // Extract the field names and types.
    let field_names = fields.iter().map(|field| &field.ident).collect::<Vec<_>>();
    let field_types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();

    // Get the name of the source type.
    let source_type = quote! { #input };

    // Generate the `impl From` implementation.
    let output = quote! {
        impl From<#source_type> for #input {
            fn from(val: #source_type) -> Self {
                Self {
                    #(
                        #field_names: match val.#field_names {
                            #field_types => val.#field_names,
                            _ => panic!("Field type mismatch"),
                        },
                    )*
                }
            }
        }
    };

    // Convert the generated code back into tokens and return them.
    output.into()
}
