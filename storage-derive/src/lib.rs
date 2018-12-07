extern crate proc_macro;
extern crate quote;
extern crate syn;

use quote::quote;

use proc_macro::TokenStream;

///
/// A small derive procedure macro to implement **Storage** trait.
///
/// ```
///  #[derive(Storage)]
///  #[store_at = "meta.json"]
///  pub struct Meta {
///      pub token: String,
///  }
/// ```
/// The `store_at` attribute is required.
///
#[proc_macro_derive(Storage, attributes(store_at))]
pub fn derive_storage(input: TokenStream) -> TokenStream {
    let parse = syn::parse(input).unwrap();
    impl_derive_storage_macro(&parse)
}

fn impl_derive_storage_macro(derive_input: &syn::DeriveInput) -> TokenStream {
    let name = &derive_input.ident;

    let store_at: &syn::MetaNameValue = &derive_input.attrs.iter()
        .find_map(get_meta_items).expect("Expect a store_at attribute");
    let file_name = &store_at.lit;
    let gen = quote! {
        impl Storage for #name {
            fn get_file_name() -> String {
                stringify!(#file_name).to_string()
            }
        }
    };
    gen.into()
}

fn get_meta_items(attr: &syn::Attribute) -> Option<syn::MetaNameValue> {
    if let Ok(syn::Meta::NameValue(meta_name_value)) = attr.parse_meta() {
        if meta_name_value.ident == "store_at" {
            Some(meta_name_value)
        } else {
            None
        }
    } else {
        None
    }
}
