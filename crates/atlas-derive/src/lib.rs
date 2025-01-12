mod url;
mod try_from_map;

use proc_macro::TokenStream;

#[proc_macro_derive(AtlasURL, attributes(url))]
pub fn derive_atlas_url(input: TokenStream) -> TokenStream {
    url::derive_atlas_url_impl(input)
}

#[proc_macro_derive(TryFromMap)]
pub fn derive_try_from_map(input: TokenStream) -> TokenStream {
    try_from_map::derive_try_from_map_impl(input)
}
