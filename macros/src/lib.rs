mod selector;

use proc_macro::TokenStream;

#[proc_macro_derive(Selector, attributes(selector))]
pub fn derive_selector(input: TokenStream) -> TokenStream {
	selector::derive_selector(input)
}

#[proc_macro_attribute]
pub fn selector(attr: TokenStream, item: TokenStream) -> TokenStream {
	selector::selector(attr, item)
}
