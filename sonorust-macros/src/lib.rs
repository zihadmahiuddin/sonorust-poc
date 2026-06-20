mod memory;
mod opcode;
mod utils;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote2::proc_macro2;

#[proc_macro]
pub fn generate_memory_access(input: TokenStream) -> TokenStream {
    let mut t = TokenStream2::new();
    memory::generate_memory_access(&mut t, syn::parse_macro_input!(input));
    t.into()
}

#[proc_macro]
pub fn opcode_registry(input: TokenStream) -> TokenStream {
    opcode::opcode_registry(syn::parse_macro_input!(input)).into()
}
