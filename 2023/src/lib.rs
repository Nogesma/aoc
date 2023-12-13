use proc_macro::TokenStream;
use syn::{parse_macro_input, LitInt};
#[proc_macro]
pub fn generate_days(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitInt).base10_parse().unwrap();

    let mut imports = String::with_capacity(11 * input as usize);
    let mut match_function = String::with_capacity(76 + 22 * input as usize + 35);
    let mut all = Vec::with_capacity(input);

    match_function += "fn run_day(idx:&str,test:bool){ match idx{";
    for i in 1..=input {
        imports += format!("mod day{i:02};\n").as_str();
        match_function += format!("\"day{i:02}\" => run(idx,test,day{i:02}::main),").as_str();
        all.push(format!("day{i:02}"));
    }
    match_function +=
        format!("\"all\" => {{{all:?}.iter().for_each(|x| run_day(x, test));}},").as_str();
    match_function += "_ => panic!(\"Invalid day: {idx}\")}}";

    (imports + &match_function).parse().unwrap()
}
