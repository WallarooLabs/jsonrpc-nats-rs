use darling::FromDeriveInput;
use darling::FromMeta;
use syn::parse_quote;
use syn::parse_str;
use syn::DeriveInput;
use syn::Ident;
use syn::Path;
use syn::Type;

#[proc_macro_derive(JsonRpc2, attributes(jsonrpc))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive2(input.into()).into()
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(jsonrpc))]
struct JsonRpcAttrs {
    method: String,
    request: Option<Type>,
    response: Option<Type>,
    error: Option<Type>,
    #[darling(default)]
    crates: Crates,
    #[darling(default)]
    client: bool,
}

#[derive(Debug, FromMeta)]
struct Crates {
    #[darling(default = "Self::default_jsonrpc")]
    jsonrpc: Path,
    #[darling(default = "Self::default_serde_json")]
    serde_json: Path,
}

impl JsonRpcAttrs {
    fn method(&self) -> &str {
        &self.method
    }

    fn request(&self, name: &Ident) -> Type {
        self.request
            .clone()
            .unwrap_or_else(|| parse_str(&format!("{name}Request")).unwrap())
    }

    fn response(&self, name: &Ident) -> Type {
        self.response
            .clone()
            .unwrap_or_else(|| parse_str(&format!("{name}Response")).unwrap())
    }

    fn error(&self, name: &Ident) -> Type {
        self.error
            .clone()
            .unwrap_or_else(|| syn::parse_str(&format!("{name}Error")).unwrap())
    }

    fn jsonrpc(&self) -> &Path {
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
    fn default_jsonrpc() -> Path {
        parse_quote!(::jsonrpc)
    }

    fn default_serde_json() -> Path {
        parse_quote!(::serde_json)
    }
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
    let client = attrs.client.then(|| derive_client(&ast, &attrs));

    quote::quote!(
        #jsonrpc2
        #client
    )
}

fn derive_jsonrpc2(ast: &DeriveInput, attrs: &JsonRpcAttrs) -> proc_macro2::TokenStream {
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

fn derive_client(ast: &DeriveInput, attrs: &JsonRpcAttrs) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let jsonrpc = attrs.jsonrpc();
    let clientext = Ident::new(&format!("{}Ext", name), name.span());
    let serde_json = &attrs.crates.serde_json;

    let method = Ident::new(attrs.method(), name.span());
    let request = attrs.request(name);
    let response = attrs.response(name);
    let error = attrs.error(name);

    let request = match request {
        Type::Tuple(tuple) if tuple.elems.is_empty() => None,
        other => Some(other),
    };

    let method_params = request.map(
        |request| quote::quote!(request: impl ::core::convert::Into<::core::option::Option<#request>> + ::core::marker::Send,),
    );

    let call_params = if method_params.is_some() {
        quote::quote!(request)
    } else {
        quote::quote!(None)
    };

    quote::quote!(
        #[#jsonrpc::async_trait]
        pub trait #clientext<T>
        where
            T: #jsonrpc::JsonRpc2Service<#jsonrpc::Request, Response = #jsonrpc::Response>,
            T::Error: ::core::convert::From<#serde_json::Error>,
        {
            async fn #method(
                &self,
                #method_params
            ) -> ::core::result::Result<::core::result::Result<#response, #error>, T::Error>;
        }

        #[#jsonrpc::async_trait]
        impl<T> #clientext<T> for #jsonrpc::AsyncClient<T>
        where
            T: #jsonrpc::JsonRpc2Service<#jsonrpc::Request, Response = #jsonrpc::Response>,
            T::Error: ::core::convert::From<#serde_json::Error>,
        {
            async fn #method(
                &self,
                #method_params
            ) -> ::core::result::Result<::core::result::Result<#response, #error>, T::Error> {
                self.call::<#name>(#call_params).await
            }
        }

    )
}
