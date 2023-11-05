use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

const BEGIN: &str = "$(";
const END: &str = ")";

#[proc_macro]
pub fn texformat(input: TokenStream) -> TokenStream {
  let format_string = parse_macro_input!(input as LitStr);
  let interpolated = format_string.value().replace(BEGIN, "{").replace(END, "}");
  TokenStream::from(quote! { format!(#interpolated) })
}
