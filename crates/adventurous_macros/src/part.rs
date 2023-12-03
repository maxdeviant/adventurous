use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::Token;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Part {
    One,
    Two,
}

impl Part {
    pub fn answer_fn_ident(&self) -> syn::Ident {
        let part_number = match self {
            Part::One => "one",
            Part::Two => "two",
        };

        syn::parse_str::<syn::Ident>(&format!("part_{part_number}_answer"))
            .expect("failed to create identifier")
    }
}

pub struct PartInput {
    answer: Option<String>,
}

impl PartInput {
    pub fn empty() -> Self {
        Self { answer: None }
    }
}

impl Parse for PartInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let Some(ident) = input.parse::<syn::Ident>().ok() else {
            return Ok(Self::empty());
        };

        if ident.to_string() == "answer" {
            let _ = input.parse::<Token![=]>()?;
            let lit = input.parse::<syn::LitStr>()?;

            return Ok(Self {
                answer: Some(lit.value()),
            });
        }

        return Ok(Self::empty());
    }
}

pub fn part_impl(part: Part, input: PartInput, item: TokenStream) -> TokenStream {
    let answer_fn_ident = part.answer_fn_ident();

    let answer = match input.answer {
        Some(answer) => quote! { Some(#answer)},
        None => quote! { None },
    };

    quote! {
        #item

        const fn #answer_fn_ident() -> Option<&'static str> {
            #answer
        }
    }
}
