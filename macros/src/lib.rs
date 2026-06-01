use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, LitStr, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

struct LogInput {
    name: Expr,
    fmt: LitStr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for LogInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Expr>()?;

        input.parse::<Token![,]>()?;

        let fmt = input.parse::<LitStr>()?;

        let args = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Punctuated::<Expr, Token![,]>::parse_terminated(input)?
        } else {
            Punctuated::new()
        };

        Ok(Self { name, fmt, args })
    }
}

fn expand(method: &str, input: TokenStream) -> TokenStream {
    let LogInput { name, fmt, args } = parse_macro_input!(input as LogInput);
    let method = syn::Ident::new(method, proc_macro2::Span::call_site());
    let args_list = args.into_iter().collect::<Vec<_>>();

    quote! {
        fwkarq::logger::provider::Provider::get_logger(#name)
            .#method(
                format!(#fmt, #(#args_list),*)
            );
    }
    .into()
}

#[proc_macro]
pub fn debug(input: TokenStream) -> TokenStream {
    expand("debug", input)
}

#[proc_macro]
pub fn info(input: TokenStream) -> TokenStream {
    expand("info", input)
}

#[proc_macro]
pub fn warning(input: TokenStream) -> TokenStream {
    expand("warning", input)
}

#[proc_macro]
pub fn error(input: TokenStream) -> TokenStream {
    expand("error", input)
}

#[proc_macro]
pub fn critical(input: TokenStream) -> TokenStream {
    expand("critical", input)
}
