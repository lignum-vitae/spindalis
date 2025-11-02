use std::str::FromStr;

use proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn parse_simple_polynomial(input: TokenStream) -> TokenStream {
    let output =
        spindalis_core::polynomials::simple::parse_simple_polynomial(&input.to_string()).unwrap();
    TokenStream::from_str(&format!("{output:?}")).unwrap()
}
