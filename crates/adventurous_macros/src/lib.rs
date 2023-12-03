mod part;
mod test_solutions;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::part::{part_impl, Part, PartInput};
use crate::test_solutions::test_solutions_impl;

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

/// Emits test cases to test the solutions to the puzzle.
///
/// # Examples
///
/// ```
/// #[cfg(test)]
/// mod tests {
///     adventurous::test_solutions!();
/// }
/// ```
#[proc_macro]
pub fn test_solutions(_input: TokenStream) -> TokenStream {
    test_solutions_impl().into()
}
