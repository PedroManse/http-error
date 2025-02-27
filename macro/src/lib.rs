extern crate proc_macro;
use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream as TS2;
use quote::quote;
use syn::{*, parse::{Parse, ParseStream}};

trait Render {
    fn render(self) -> TS2;
}

#[derive(Debug)]
struct HttpError {
    mod_name: Option<Ident>,
    name: Ident,
    items: Vec<ErrorInfo>,
}

#[derive(Debug)]
struct ErrorInfo {
    name: Ident,
    item: ErrorItem,
}

impl Parse for ErrorInfo {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![=] = input.parse()?;
        let item: ErrorItem = input.parse()?;
        Ok(ErrorInfo{
            name, item
        })
    }
}

#[derive(Debug)]
enum ErrorItem {
    Alias(Ident), // don't create type
    Tuple(ExprTuple), // create type
    Struct(ExprStruct), // create type // manually add prefix
}

impl Parse for ErrorItem{
    fn parse(input: ParseStream) -> Result<Self> {
        //let item_alias = input.parse::<Ident>().map(ErrorItem::Alias);
        let item_tuple = input.parse::<ExprTuple>().map(ErrorItem::Tuple);
        let item_struct = input.parse::<ExprStruct>().map(ErrorItem::Struct);
        item_tuple.or(item_struct)
    }
}

impl Render for ErrorInfo {
    fn render(self) -> TS2 {
        let name = self.name;
        match self.item {
            ErrorItem::Alias(i)=>quote! {type #name = #i;},
            ErrorItem::Tuple(e)=>quote! {struct #name #e;},
            ErrorItem::Struct(s)=>quote! {struct #name #s}
        }
    }

}

#[proc_macro]
pub fn http_error(item: TS) -> TS {
    let e: ErrorInfo = syn::parse(item).unwrap();
    e.render().into()
}

