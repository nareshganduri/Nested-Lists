extern crate proc_macro;
extern crate nested_list;

use proc_macro2::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::{quote, quote_spanned};
use syn::parse_macro_input;
use syn::spanned::Spanned;

use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::Bracket;
use syn::{bracketed, Expr, Token};

struct ExprNestedList {
    items: Punctuated<ExprListItem, Token![,]>,
}

impl Parse for ExprNestedList {
    fn parse(input: ParseStream) -> Result<Self> {
        let items;

        bracketed!(items in input);

        Ok(ExprNestedList {
            items: items.parse_terminated(ExprListItem::parse)?,
        })
    }
}

enum ExprListItem {
    Item(Expr),
    List(ExprNestedList),
}

impl Parse for ExprListItem {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Bracket) {
            Ok(ExprListItem::List(input.parse()?))
        } else {
            Ok(ExprListItem::Item(input.parse()?))
        }
    }
}

#[proc_macro_hack]
pub fn nested_list(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let list = parse_macro_input!(input as ExprNestedList);

    let items = list.items.iter().map(|f| gen_list_item(f));

    let expanded = quote! {
        nested_list::NestedList(vec![
            #( #items ),*
        ])
    };

    expanded.into()
}

fn gen_list_item(item: &ExprListItem) -> TokenStream {
    match item {
        &ExprListItem::Item(ref x) => {
            let expanded = quote_spanned! {x.span() =>
                nested_list::NestedListItem::Item(#x)
            };

            expanded.into()
        }
        &ExprListItem::List(ref x) => {
            let items = x.items.iter().map(|f| gen_list_item(f));

            let expanded = quote! {
                nested_list::NestedListItem::List(NestedList(vec![
                    #( #items ),*
                ]))
            };

            expanded.into()
        }
    }
}
