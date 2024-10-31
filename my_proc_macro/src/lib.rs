extern crate proc_macro2;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ext::IdentExt, DeriveInput};

// function-like macros
#[proc_macro]
pub fn say_hello(item: TokenStream) -> TokenStream {
    println!("item: {}", item);
    "fn hello() -> String { \"Hello!\".to_string() }"
        .parse()
        .unwrap()
}

// attribute macros
#[proc_macro_attribute]
pub fn log_info(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}

// derive macros
#[proc_macro_derive(IntoURLQueryString)]
pub fn derive_into_query(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as DeriveInput);

    match generate_query_string(&input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate_query_string(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_data = match &derive_input.data {
        syn::Data::Struct(v) => v,
        _ => {
            return Err(syn::Error::new_spanned(
                &derive_input.ident,
                "Must be struct type",
            ));
        }
    };

    let struct_name = &derive_input.ident;

    let mut implementation = quote! {
        let mut query_string: String = "".to_owned();
    };

    for (index, field) in struct_data.clone().fields.into_iter().enumerate() {
        let identifier = field.ident.as_ref().unwrap();
        let identifier_string = identifier.unraw().to_string();
        implementation.extend(quote! {
            if #index == 0 {
                query_string = format!(
                    "{}{}={}",
                    query_string,
                    #identifier_string,
                    urlencoding::encode(&format!("{}",value.#identifier))
                )
            } else {
                query_string = format!(
                    "{}&{}={}",
                    query_string,
                    #identifier_string,
                    urlencoding::encode(&format!("{}",value.#identifier))
                )
            }
        });
    }

    Ok(quote! {
        #[automatically_derived]
        impl From<#struct_name> for String {
            fn from(value: #struct_name) -> Self {
                #implementation
                query_string.to_string()
            }
        }
    }
    .into())
}

#[proc_macro_derive(URLQueryGetter)]
pub fn derive_query_getter(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as DeriveInput);

    match generate_query_getters(&input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate_query_getters(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_data = match &derive_input.data {
        syn::Data::Struct(v) => v,
        _ => {
            return Err(syn::Error::new_spanned(
                &derive_input.ident,
                "Must be struct type",
            ));
        }
    };
    let struct_name = &derive_input.ident;

    let mut get_fields: Vec<proc_macro2::TokenStream> = Vec::new();
    for field in &struct_data.fields {
        let identifier = field.ident.as_ref().unwrap();
        let identifier_string = identifier.unraw().to_string();

        let method_name: proc_macro2::TokenStream =
            format!("get_{}_query", identifier_string).parse().unwrap();

        get_fields.push(quote! {
            pub fn #method_name(&self) -> String {
                format!(
                    "{}={}",
                    #identifier_string,
                    urlencoding::encode(&format!("{}",self.#identifier.clone()))
                )
            }
        });
    }

    Ok(quote! {
        impl #struct_name {
            #(#get_fields)*
        }
    }
    .into())
}
