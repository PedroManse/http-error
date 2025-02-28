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
    name: Ident,
    items: ItemSeq,
}

impl Parse for HttpError {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        let content;
        let _ = braced!(content in input);
        let items = content.parse()?;
        Ok(HttpError{
            name,
            items
        })
    }
}

impl Render for HttpError {
    fn render(self) -> TS2 {
        let name = self.name;
        let items = self.items;
        let items_names: Vec<_> = items.0.iter().map(|i|i.name.clone()).collect();
        let items_defs = items.render();
        let x = quote!{
            pub enum #name {
                #(#items_names(#items_names)),*
            }
            #items_defs
        };
        x
    }
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
            ErrorItem::Alias(i)=>quote! {pub type #name = super::#i;},
            ErrorItem::Tuple(e)=>quote! {pub struct #name #e;},
            ErrorItem::Struct(s)=>quote! {pub struct #name #s}
        }
    }
}

#[proc_macro]
pub fn http_error(item: TS) -> TS {
    let a:HttpError = syn::parse(item).unwrap();
    a.render().into()
}

