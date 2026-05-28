use proc_macro::TokenStream;

mod memory;
mod opcode;

#[proc_macro]
pub fn generate_memory_access(item: TokenStream) -> TokenStream {
    memory::generate_memory_access(item)
}

#[proc_macro]
pub fn opcode_registry(item: TokenStream) -> TokenStream {
    opcode::opcode_registry(item)
}
