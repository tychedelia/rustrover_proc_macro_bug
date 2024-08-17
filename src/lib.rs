use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, parse::Parse, parse::ParseStream, Token, parenthesized};

struct RequireAttr {
    struct_name: Ident,
    method_name: Ident,
}

impl Parse for RequireAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let struct_name: Ident = input.parse()?;
        let content;
        parenthesized!(content in input);
        let method_name: Ident = content.parse()?;
        Ok(RequireAttr { struct_name, method_name })
    }
}

#[proc_macro_derive(MyComponent, attributes(require))]
pub fn my_component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let attr = input.attrs.iter().find(|a| a.path().is_ident("require")).expect("require attribute not found");
    let require_attr = attr.parse_args::<RequireAttr>().expect("failed to parse require attribute");

    let struct_name = &require_attr.struct_name;
    let method_name = &require_attr.method_name;

    let expanded = quote! {
        impl MyComponentTrait for #name {
            fn foo() {
                let x: #struct_name = #method_name().into();
            }
        }
    };

    TokenStream::from(expanded)
}
