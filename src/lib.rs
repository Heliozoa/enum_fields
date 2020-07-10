//! Contains a proc macro attribute for adding fields to all named variants of an enum.
//! Also generates immutable and mutable accessors for the fields.

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Field, Fields, ItemEnum, Token};

struct FieldVec(Vec<Field>);

impl Parse for FieldVec {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut fields = vec![];
        while !input.is_empty() {
            fields.push(Field::parse_named(input)?);
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
        }
        Ok(FieldVec(fields))
    }
}

#[proc_macro_attribute]
pub fn add(arg: TokenStream, input: TokenStream) -> TokenStream {
    let input_fields = syn::parse_macro_input!(arg as FieldVec);
    let mut input_enum = syn::parse_macro_input!(input as ItemEnum);

    // add fields to enum and store named variants
    let mut named_variants = vec![];
    let mut unnamed_variants = false;
    for variant in input_enum.variants.iter_mut() {
        match &mut variant.fields {
            Fields::Named(named) => {
                named.named.extend(input_fields.0.clone());
                named_variants.push(variant.ident.clone());
            }
            _ => unnamed_variants = true,
        }
    }

    // create an impl block with accessors for the fields
    let enum_ident = &input_enum.ident;
    let fields = input_fields.0.into_iter().map(|f| (f.ident.unwrap(), f.ty));

    let accessors = fields.map(|(field_id, field_type)| {
        // if there are unnamed variants, the accessors will return options
        let (fn_return_type, mut_return_type, match_return, none_arm) = if unnamed_variants {
            (
                quote::quote! {Option<&#field_type>},
                quote::quote! {Option<&mut #field_type>},
                quote::quote! {Some(#field_id)},
                quote::quote! {_ => None},
            )
        } else {
            (
                quote::quote! {&#field_type},
                quote::quote! {&mut #field_type},
                quote::quote! {#field_id},
                quote::quote! {},
            )
        };

        let field_id_mut = quote::format_ident!("{}_mut", field_id);
        quote::quote! {
            fn #field_id(&self) -> #fn_return_type {
                match self {
                    #(Self::#named_variants { #field_id, .. } => #match_return,)*
                    #none_arm
                }
            }
            fn #field_id_mut(&mut self) -> #mut_return_type {
                match self {
                    #(Self::#named_variants { #field_id, .. } => #match_return,)*
                    #none_arm
                }
            }
        }
    });

    let impl_block = quote::quote! {
        impl #enum_ident {
            #(#accessors)*
        }
    };

    TokenStream::from(quote::quote! {
        #input_enum
        #impl_block
    })
}
