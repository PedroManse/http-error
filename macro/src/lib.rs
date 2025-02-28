extern crate proc_macro;
use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream as TS2;
use quote::quote;
use syn::{*, parse::{Parse, ParseStream}};

use self::punctuated::Punctuated;

trait Render {
    fn render(self) -> TS2;
}

#[derive(Debug)]
struct HttpError {
    mod_name: Option<Ident>,
    name: Ident,
    items: ItemSeq,
}

#[derive(Debug)]
struct ItemSeq(Vec<ErrorInfo>);
impl Parse for ItemSeq {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut out = vec![];
        loop {
            out.push(input.parse()?);
            match input.parse::<Token![,]>() {
                Ok(_)=>(),
                Err(_)=>break
            }
        }
        Ok(ItemSeq(out))
    }
}

impl Render for ItemSeq {
    fn render(self) -> TS2 {
        let xs = self.0.into_iter().map(|x|x.render());
        quote! {
            #(#xs)*
        }
    }
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
    Struct(FieldsNamed), // create type // manually add prefix
}


impl Parse for ErrorItem{
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            input.parse().map(ErrorItem::Alias)
        } else if lookahead.peek(token::Paren) {
            input.parse().map(ErrorItem::Tuple)
        } else if lookahead.peek(token::Brace) {
            input.parse().map(ErrorItem::Struct)
        } else {
            Err(lookahead.error())
        }
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
    let a = syn::parse(item);
    let e: ItemSeq = a.unwrap();
    let x = e.render();
    x.into()
}

