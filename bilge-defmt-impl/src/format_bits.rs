use proc_macro_error2::abort_call_site;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident};

pub(crate) fn format_bits(item: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse2(item).unwrap_or_else(|_| unreachable!());
    let name = &derive_input.ident;
    let name_str = name.to_string();
    let struct_data = match derive_input.data {
        Data::Struct(s) => s,
        Data::Enum(_) => abort_call_site!("use derive(defmt::Format) for enums"),
        Data::Union(_) => unreachable!(),
    };

    let fmt_impl = match struct_data.fields {
        Fields::Named(fields) => {
            let mut fmt_string = name_str.to_string();
            fmt_string.push_str(" {{ ");

            let mut fields = fields.named.iter().peekable();

            let mut calls = Vec::new();
            while let Some(f) = fields.next() {
                // We can unwrap since this is a named field
                let call = f.ident.as_ref().unwrap();
                let name = call.to_string();
                calls.push(quote!(self.#call(),));

                if fields.peek().is_some() {
                    fmt_string.push_str(&format!("{name}: {{}}, "));
                } else {
                    fmt_string.push_str(&format!("{name}: {{}} "));
                }
            }

            fmt_string.push_str("}}");

            quote! {
                ::defmt::write!(fmt, #fmt_string, #(#calls)*)
            }
        }
        Fields::Unnamed(fields) => {
            let mut fmt_string = name_str.to_string();
            fmt_string.push_str("(");

            let mut fields = fields.unnamed.iter().enumerate().peekable();

            let mut calls = Vec::new();
            while let Some((idx, _)) = fields.next() {
                // We can unwrap since this is a named field
                let call: Ident =
                    syn::parse_str(&format!("val_{idx}")).unwrap_or_else(|_| unreachable!());
                calls.push(quote!(self.#call(),));

                if fields.peek().is_some() {
                    fmt_string.push_str(&format!("{{}}, "));
                } else {
                    fmt_string.push_str(&format!("{{}}"));
                }
            }

            fmt_string.push_str(")");

            quote! {
                ::defmt::write!(fmt, #fmt_string, #(#calls)*)
            }
        }
        Fields::Unit => todo!("this is a unit struct, which is not supported right now"),
    };

    quote! {
        impl ::defmt::Format for #name {
            fn format(&self, fmt: ::defmt::Formatter) {
                #fmt_impl
            }
        }
    }
}
