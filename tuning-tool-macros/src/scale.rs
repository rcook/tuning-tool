use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn scale(input: TokenStream) -> TokenStream {
    fn make_literals(input: &TokenStream) -> Vec<String> {
        let mut strs = Vec::new();
        let mut iter = input.clone().into_iter().peekable();
        while let Some(tt) = iter.next() {
            if iter.next_if(|s| s.to_string() == "/").is_some() {
                let denom_tt = iter.next().expect("Malformed ratio?");
                strs.push(format!("{tt}/{denom_tt}"));
            } else {
                strs.push(tt.to_string())
            }
        }
        strs
    }

    let parse_exprs = make_literals(&input)
        .iter()
        .map(|s| {
            quote! {
                #s.parse::<Interval>()?
            }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        [#(#parse_exprs),*]
    };

    output.into()
}
