use proc_macro::TokenStream;

mod core;

#[proc_macro_derive(ConfigSection)]
pub fn config_section_derive(input: TokenStream) -> TokenStream {
    // let ast = syn::parse(input).unwrap();
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    core::config2::impl_config_section(&ast)
}

#[proc_macro_derive(Config)]
pub fn config_derive(input: TokenStream) -> TokenStream {
    // let ast = syn::parse(input).unwrap();
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    core::config2::impl_config(&ast)
}
