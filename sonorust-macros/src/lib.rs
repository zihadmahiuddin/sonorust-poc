use proc_macro::TokenStream;

mod memory;
mod opcode;

#[proc_macro]
pub fn generate_memory_access(input: TokenStream) -> TokenStream {
    memory::generate_memory_access(syn::parse_macro_input!(input)).into()
}

#[proc_macro]
pub fn opcode_registry(input: TokenStream) -> TokenStream {
    opcode::opcode_registry(syn::parse_macro_input!(input))
}
