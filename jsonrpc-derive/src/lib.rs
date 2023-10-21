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
    fn method(&self) -> &str {
        &self.method
    }

    fn request(&self, name: &Ident) -> Ident {
        if let Some(request) = &self.request {
            Ident::new(request, name.span())
        } else {
            Ident::new(&format!("{}Request", name), name.span())
        }
    }

    fn response(&self, name: &Ident) -> Ident {
        if let Some(response) = &self.response {
            Ident::new(response, name.span())
        } else {
            Ident::new(&format!("{}Response", name), name.span())
        }
    }

    fn error(&self, name: &Ident) -> Ident {
        if let Some(error) = &self.error {
            Ident::new(error, name.span())
        } else {
            Ident::new(&format!("{}Error", name), name.span())
        }
    }

    fn jsonrpc(&self) -> &Path {
        &self.crates.jsonrpc
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
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive2(input.into()).into()
}

fn derive2(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let ast: DeriveInput = match syn::parse2(input) {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };

    let attrs = match JsonRpcAttrs::from_derive_input(&ast) {
        Ok(attrs) => attrs,
        Err(err) => return err.write_errors(),
    };

    let jsonrpc2 = derive_jsonrpc2(&ast, &attrs);

    quote!(
        #jsonrpc2
    )
}

fn derive_jsonrpc2(ast: &DeriveInput, attrs: &JsonRpcAttrs) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let jsonrpc = attrs.jsonrpc();
    let method = attrs.method();
    let request = attrs.request(name);
    let response = attrs.response(name);
    let error = attrs.error(name);

    quote!(
        #[automatically_derived]
        impl #jsonrpc::JsonRpc2 for #name {
            const METHOD: &'static str = #method;
            type Request = #request;
            type Response = #response;
            type Error = #error;
        }
    )
}
