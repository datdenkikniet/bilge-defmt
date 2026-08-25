use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
mod format_bits;
use manyhow::manyhow;

/// Generate an `impl defmt::Debug` for bitfield structs.
///
/// Use a normal #[derive(defmt::Derive)] for enums.
#[manyhow(proc_macro_derive(FormatBits, attributes(bitsize_internal)))]
pub fn format_bits(item: TokenStream) -> manyhow::Result<TokenStream2> {
    format_bits::format_bits(item.into())
}
