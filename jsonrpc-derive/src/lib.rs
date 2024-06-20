use darling::export::syn;
use darling::FromDeriveInput;
use darling::FromMeta;

use client::Client;

mod client;

#[proc_macro_derive(JsonRpc2, attributes(jsonrpc))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive2(input.into()).into()
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(jsonrpc))]
struct JsonRpcAttrs {
    method: String,
    request: Option<syn::Type>,
    response: Option<syn::Type>,
    error: Option<syn::Type>,
    #[darling(default)]
    crates: Crates,
    client: Client,
}

#[derive(Debug, FromMeta)]
struct Crates {
    #[darling(default = "Self::default_jsonrpc")]
    jsonrpc: syn::Path,
    #[darling(default = "Self::default_serde_json")]
    serde_json: syn::Path,
}

impl JsonRpcAttrs {
    fn method(&self) -> &str {
        &self.method
    }

    fn request(&self, name: &syn::Ident) -> syn::Type {
        self.request
            .clone()
            .unwrap_or_else(|| syn::parse_str(&format!("{name}Request")).unwrap())
    }

    fn response(&self, name: &syn::Ident) -> syn::Type {
        self.response
            .clone()
            .unwrap_or_else(|| syn::parse_str(&format!("{name}Response")).unwrap())
    }

    fn error(&self, name: &syn::Ident) -> syn::Type {
        self.error
            .clone()
            .unwrap_or_else(|| syn::parse_str(&format!("{name}Error")).unwrap())
    }

    fn jsonrpc(&self) -> &syn::Path {
        &self.crates.jsonrpc
    }
}

impl Default for Crates {
    fn default() -> Self {
        Self {
            jsonrpc: Self::default_jsonrpc(),
            serde_json: Self::default_serde_json(),
        }
    }
}

impl Crates {
    fn default_jsonrpc() -> syn::Path {
        syn::parse_quote!(::jsonrpc)
    }

    fn default_serde_json() -> syn::Path {
        syn::parse_quote!(::serde_json)
    }
}

fn derive2(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let ast: syn::DeriveInput = match syn::parse2(input) {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    let attrs = match JsonRpcAttrs::from_derive_input(&ast) {
        Ok(attrs) => attrs,
        Err(err) => return err.write_errors(),
    };

    let jsonrpc2 = derive_jsonrpc2(&ast, &attrs);
    let client = attrs.client.derive(&ast, &attrs);

    quote::quote!(
        #jsonrpc2
        #client
    )
}

fn derive_jsonrpc2(ast: &syn::DeriveInput, attrs: &JsonRpcAttrs) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let jsonrpc = attrs.jsonrpc();
    let method = attrs.method();
    let request = attrs.request(name);
    let response = attrs.response(name);
    let error = attrs.error(name);

    quote::quote!(
        #[automatically_derived]
        impl #jsonrpc::JsonRpc2 for #name {
            const METHOD: &'static str = #method;
            type Request = #request;
            type Response = #response;
            type Error = #error;
        }
    )
}
