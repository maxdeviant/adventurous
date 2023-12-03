use proc_macro2::TokenStream;
use quote::quote;

use crate::part::{Part, PART_ONE_ANSWER, PART_TWO_ANSWER};

pub fn test_solutions_impl() -> TokenStream {
    let part_one_test = test_solution(Part::One);
    let part_two_test = test_solution(Part::Two);

    quote! {
        #part_one_test
        #part_two_test
    }
}

fn test_solution(part: Part) -> TokenStream {
    let answer = match part {
        Part::One => PART_ONE_ANSWER.get().and_then(|answer| answer.clone()),
        Part::Two => PART_TWO_ANSWER.get().and_then(|answer| answer.clone()),
    };

    let part_fn = part.part_fn_ident();
    let test_fn_ident = {
        let part_number = match part {
            Part::One => "one",
            Part::Two => "two",
        };

        syn::parse_str::<syn::Ident>(&format!("test_part_{part_number}_solution"))
            .expect("failed to create identifier")
    };

    match answer {
        Some(answer) => {
            quote! {
                #[test]
                fn #test_fn_ident() -> anyhow::Result<()> {
                    adventurous::test::test_solution("input.txt", #part_fn, #answer)
                }
            }
        }
        None => {
            quote! {
                #[test]
                #[ignore = "not yet solved"]
                fn #test_fn_ident() {}
            }
        }
    }
}
