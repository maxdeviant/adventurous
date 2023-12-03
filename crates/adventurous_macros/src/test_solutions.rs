use proc_macro2::TokenStream;
use quote::quote;

use crate::part::Part;

pub fn test_solutions_impl() -> TokenStream {
    let part_one_answer_fn_ident = Part::One.answer_fn_ident();
    let part_two_answer_fn_ident = Part::Two.answer_fn_ident();

    quote! {
        #[test]
        fn test_part_one_solution() -> anyhow::Result<()> {
            match #part_one_answer_fn_ident() {
                Some(answer) => adventurous::test::test_solution("input.txt", part_one, answer),
                None => {
                    eprintln!("Part One not solved");
                    Ok(())
                }
            }
        }

        #[test]
        fn test_part_two_solution() -> anyhow::Result<()> {
            match #part_two_answer_fn_ident() {
                Some(answer) => adventurous::test::test_solution("input.txt", part_two, answer),
                None => {
                    eprintln!("Part Two not solved");
                    Ok(())
                }
            }
        }
    }
}
