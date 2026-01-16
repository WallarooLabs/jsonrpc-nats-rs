use super::*;

#[derive(Debug)]
#[expect(clippy::large_enum_variant)]
pub(crate) enum Client {
    Skip,
    Default,
    NewType(syn::Type),
}

impl Client {
    pub(crate) fn derive(
        &self,
        ast: &syn::DeriveInput,
        attrs: &JsonRpcAttrs,
    ) -> Option<proc_macro2::TokenStream> {
        match self {
            Self::Skip => None,
            Self::Default => Some(derive_client(ast, attrs, None)),
            Self::NewType(r#type) => Some(derive_client(ast, attrs, Some(r#type))),
        }
    }
}

impl FromMeta for Client {
    fn from_none() -> Option<Self> {
        Some(Self::Skip)
    }

    fn from_word() -> darling::Result<Self> {
        Ok(Self::Default)
    }

    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        let r#type = litstr(value)?.parse()?;
        Ok(Self::NewType(r#type))
    }
}

fn litstr(value: &syn::Lit) -> darling::Result<&syn::LitStr> {
    match value {
        syn::Lit::Str(value) => Ok(value),
        _ => Err(darling::Error::unexpected_lit_type(value)),
    }
}

fn derive_client(
    ast: &syn::DeriveInput,
    attrs: &JsonRpcAttrs,
    client: Option<&syn::Type>,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let jsonrpc = attrs.jsonrpc();
    let clientext = syn::Ident::new(&format!("{}Ext", name), name.span());
    let serde_json = &attrs.crates.serde_json;

    let method = syn::Ident::new(attrs.method(), name.span());
    let request = attrs.request(name);
    let response = attrs.response(name);
    let error = attrs.error(name);

    let request = match request {
        syn::Type::Tuple(tuple) if tuple.elems.is_empty() => None,
        other => Some(other),
    };

    let method_params = request.map(
        |request| quote::quote!(request: impl #jsonrpc::export::Into<#jsonrpc::export::Option<#request>> + #jsonrpc::export::Send,),
    );

    let call_params = if method_params.is_some() {
        quote::quote!(request)
    } else {
        quote::quote!(None)
    };

    let client = client.map_or_else(
        || quote::quote!(#jsonrpc::AsyncClient<T>),
        |r#type| quote::quote!(#r#type),
    );

    quote::quote!(
        pub trait #clientext<T>: #jsonrpc::export::AsRef<#jsonrpc::AsyncClient<T>>
        where
            T: #jsonrpc::JsonRpc2Service<#jsonrpc::Request, Response = #jsonrpc::Response>,
            T::Error: #jsonrpc::export::From<#serde_json::Error>,
        {
            fn #method(
                &self,
                #method_params
            ) -> impl #jsonrpc::export::Future<Output = #jsonrpc::export::Result<jsonrpc::export::Result<#response, #error>, T::Error>> + #jsonrpc::export::Send;
        }

        impl<T> #clientext<T> for #client
        where
            T: #jsonrpc::JsonRpc2Service<#jsonrpc::Request, Response = #jsonrpc::Response>,
            T::Error: #jsonrpc::export::From<#serde_json::Error>,
        {
            async fn #method(
                &self,
                #method_params
            ) -> #jsonrpc::export::Result<#jsonrpc::export::Result<#response, #error>, T::Error> {
                self.as_ref().call::<#name>(#call_params).await
            }
        }

    )
}
