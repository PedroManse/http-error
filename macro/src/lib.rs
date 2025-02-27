extern crate proc_macro;
use proc_macro::TokenStream;

struct HttpError {
    mod_name: Option<String>,
    name: String,
    Items: Vec<ErrorInfo>,
}

struct ErrorInfo {
    name: String,
    item: ErrorItem,
}

enum ErrorItem {
    Alias(String), // don't create type
    Tuple(String), // create type
    Struct(String), // create type
}

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
