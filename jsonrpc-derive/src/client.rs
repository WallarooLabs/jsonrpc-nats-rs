use super::*;

#[derive(Debug)]
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
            Self::NewType(r#type) => {
                let new_type = derive_new_type(r#type, attrs);
                let client = derive_client(ast, attrs, Some(r#type));
                Some(quote::quote!(
                    #new_type
                    #client
                ))
            }
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

fn derive_new_type(r#type: &syn::Type, attrs: &JsonRpcAttrs) -> proc_macro2::TokenStream {
    let jsonrpc = attrs.jsonrpc();
    quote::quote!(
        pub struct #r#type<T>(pub #jsonrpc::AsyncClient<T>);
        impl<T> ::core::convert::From<#jsonrpc::AsyncClient<T>> for #r#type<T> {
            fn from(client: #jsonrpc::AsyncClient<T>) -> Self {
                Self(client)
            }
        }
        impl<T> ::core::ops::Deref for #r#type<T> {
            type Target = #jsonrpc::AsyncClient<T>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    )
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
        |request| quote::quote!(request: impl ::core::convert::Into<::core::option::Option<#request>> + ::core::marker::Send,),
    );

    let call_params = if method_params.is_some() {
        quote::quote!(request)
    } else {
        quote::quote!(None)
    };

    let client = client.map_or_else(
        || quote::quote!(#jsonrpc::AsyncClient<T>),
        |r#type| quote::quote!(#r#type<T>),
    );

    quote::quote!(
        pub trait #clientext<T>
        where
            T: #jsonrpc::JsonRpc2Service<#jsonrpc::Request, Response = #jsonrpc::Response>,
            T::Error: ::core::convert::From<#serde_json::Error>,
        {
            fn #method(
                &self,
                #method_params
            ) -> impl ::core::future::Future<Output = ::core::result::Result<::core::result::Result<#response, #error>, T::Error>> + ::core::marker::Send;
        }

        impl<T> #clientext<T> for #client
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
