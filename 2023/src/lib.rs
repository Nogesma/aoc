use proc_macro::TokenStream;
use syn::{parse_macro_input, LitInt};
#[proc_macro]
pub fn generate_days(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt).base10_parse().unwrap();

    let mut str = String::from(
        "fn get_day_fn(idx:&str) -> impl Fn(String) -> (String, String) { match idx {",
    );
    for i in 1..=input {
        str += format!("\"day{i:02}\" => day{i:02}::main,").as_str();
    }
    str += "_ => panic!(\"Invalid day: {idx}\")}}";

    str.parse().unwrap()
}
