use crate::tokenization::impl_tokenize;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod tokenization;

#[proc_macro_derive(Tokenize, attributes(skip, literal, regex))]
pub fn derive_tokenize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let tokens = impl_tokenize(input);

    tokens.into()
}

