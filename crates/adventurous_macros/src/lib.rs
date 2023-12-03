mod part;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::part::{part_impl, Part, PartInput};

#[proc_macro_attribute]
pub fn part_one(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(attr as PartInput);

    part_impl(Part::One, input, item.into()).into()
}

#[proc_macro_attribute]
pub fn part_two(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(attr as PartInput);

    part_impl(Part::Two, input, item.into()).into()
}
