use syn::Ident;
use syn::Path;
// use syn::Data;
use syn::DeriveInput;
// use syn::Fields;
use darling::FromDeriveInput;
use darling::FromMeta;
use quote::quote;
use syn::parse_quote;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(jsonrpc))]
struct JsonRpcAttrs {
    method: String,
    request: Option<String>,
    response: Option<String>,
    error: Option<String>,
    #[darling(default)]
    crates: Crates,
}

#[derive(Debug, FromMeta)]
struct Crates {
    #[darling(default = "Self::default_jsonrpc")]
    jsonrpc: Path,
    #[darling(default = "Self::default_std")]
    _std: Path,
}

impl JsonRpcAttrs {
    fn extract(self, name: &Ident) -> (String, Ident, Ident, Ident, Crates) {
        let request = self.request.unwrap_or_else(|| format!("{}Request", name));
        let response = self.response.unwrap_or_else(|| format!("{}Response", name));
        let error = self.error.unwrap_or_else(|| format!("{}Error", name));
        (
            self.method,
            Ident::new(&request, name.span()),
            Ident::new(&response, name.span()),
            Ident::new(&error, name.span()),
            self.crates,
        )
    }
}

impl Default for Crates {
    fn default() -> Self {
        Self {
            jsonrpc: Self::default_jsonrpc(),
            _std: Self::default_std(),
        }
    }
}

impl Crates {
    fn default_jsonrpc() -> Path {
        parse_quote!(::jsonrpc)
    }

    fn default_std() -> Path {
        parse_quote!(::std)
    }
}

#[proc_macro_derive(JsonRpc2, attributes(jsonrpc))]
pub fn derive_jsonrpc2(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_jsonrpc2_macro(input.into()).into()
}

fn impl_jsonrpc2_macro(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let ast: DeriveInput = match syn::parse2(input) {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    let attrs = match JsonRpcAttrs::from_derive_input(&ast) {
        Ok(attrs) => attrs,
        Err(err) => return err.write_errors(),
    };

    let name = &ast.ident;

    let (method, request, response, error, crates) = attrs.extract(name);
    let jsonrpc = crates.jsonrpc;

    quote!(
        impl #jsonrpc::JsonRpc2 for #name {
            const METHOD: &'static str = #method;
            type Request = #request;
            type Response = #response;
            type Error = #error;
        }
    )
}
