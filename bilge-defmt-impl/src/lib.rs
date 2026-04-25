use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;
mod format_bits;

/// Generate an `impl defmt::Debug` for bitfield structs.
///
/// Use a normal #[derive(defmt::Derive)] for enums.
#[proc_macro_error]
#[proc_macro_derive(FormatBits, attributes(bitsize_internal))]
pub fn format_bits(item: TokenStream) -> TokenStream {
    format_bits::format_bits(item.into()).into()
}
