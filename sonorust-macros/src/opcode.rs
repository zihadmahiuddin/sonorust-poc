extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::Token;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, braced};

struct OpcodeRegistryInput {
    entries: Vec<OpcodeEntry>,
}

struct OpcodeEntry {
    name: Ident,
    fields: Vec<OpcodeField>,
}

enum OpcodeField {
    Normal(Ident),
    Rest(Ident),
}

impl Parse for OpcodeRegistryInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut entries = Vec::new();
        while !input.is_empty() {
            entries.push(input.parse()?);
            input.parse::<Token![,]>()?;
        }
        Ok(Self { entries })
    }
}

impl Parse for OpcodeEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut fields = Vec::new();
        while !content.is_empty() {
            fields.push(content.parse()?);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(Self { name, fields })
    }
}

impl Parse for OpcodeField {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![..]) {
            input.parse::<Token![..]>()?;
            let ident = input.parse()?;
            Ok(OpcodeField::Rest(ident))
        } else {
            let ident = input.parse()?;
            Ok(OpcodeField::Normal(ident))
        }
    }
}

impl OpcodeRegistryInput {
    fn expand(&self) -> proc_macro2::TokenStream {
        let mut struct_defs = Vec::new();
        let mut match_arms = Vec::new();
        let mut execute_match_arms = Vec::new();
        let mut enum_variants = Vec::new();

        for entry in &self.entries {
            let name = &entry.name;

            // Ensure there is at most one rest `..` pattern per entry.
            if entry
                .fields
                .iter()
                .filter(|f| matches!(f, OpcodeField::Rest(_)))
                .count()
                > 1
            {
                panic!(
                    "Cannot have more than one rest (`..`) pattern in `{}`",
                    name
                );
            }

            let rest_position = entry
                .fields
                .iter()
                .position(|f| matches!(f, OpcodeField::Rest(_)));

            let (leading_len, trailing_len) = if let Some(pos) = rest_position {
                let leading = pos;
                let trailing = entry.fields.len() - 1 - pos;
                (leading, trailing)
            } else {
                (entry.fields.len(), 0)
            };

            // 2. Generate the struct definition.
            let struct_fields = entry.fields.iter().map(|field| match field {
                OpcodeField::Normal(ident) => quote! { pub #ident: usize, },
                OpcodeField::Rest(ident) => quote! { pub #ident: Vec<usize>, },
            });

            enum_variants.push(quote! {
                #name(#name),
            });

            struct_defs.push(quote! {
                #[derive(Debug, PartialEq)]
                pub struct #name {
                    #(#struct_fields)*
                }
            });

            // 3. CORRECTED LOGIC: Build assignments using direct indexing into partitioned vectors.
            let mut leading_idx = 0;
            let mut _trailing_idx = 0;
            let assignments = entry.fields.iter().enumerate().map(|(i, field)| {
                match field {
                    OpcodeField::Rest(ident) => {
                        quote! { #ident: rest_args }
                    }
                    OpcodeField::Normal(ident) => {
                        if rest_position.is_none_or(|pos| i < pos) {
                            // It's a leading field. Use an index into `leading_args`.
                            let idx = syn::Index::from(leading_idx);
                            leading_idx += 1;
                            quote! { #ident: leading_args[#idx] }
                        } else {
                            // It's a trailing field. Use an index into `trailing_args`.
                            let idx = syn::Index::from(_trailing_idx);
                            _trailing_idx += 1;
                            quote! { #ident: trailing_args[#idx] }
                        }
                    }
                }
            });

            // 4. Generate the match arm with the CORRECT runtime logic.
            let construction_logic = quote! {
                let mut args_vec = args;
                let total_len = args_vec.len();
                let expected_min_len = #leading_len + #trailing_len;

                if total_len < expected_min_len {
                    panic!(
                        "Incorrect number of arguments for {}. Expected at least {}, got {}.",
                        stringify!(#name), expected_min_len, total_len
                    );
                }

                // Partition the runtime vector into three distinct Vecs.
                let mut trailing_args = args_vec.split_off(total_len - #trailing_len);
                let mut rest_args = args_vec.split_off(#leading_len);
                let leading_args = args_vec; // The remainder is the leading args

                // Construct the struct using the index-based assignments.
                // Because `usize` is `Copy`, direct indexing works perfectly.
                OpCode::#name(#name {
                    #(#assignments,)*
                })
            };

            match_arms.push(quote! {
                stringify!(#name) => {
                    #construction_logic
                }
            });

            execute_match_arms.push(quote! {
                OpCode::#name(executable) => executable.execute(executor)
            });
        }

        let executor_impl = quote! {
            impl<E, M, S, T> Executable<E, M, S, T> for OpCode {
                fn execute(&self, executor: E) -> (E, f64)
                where
                    E: Executor<M, S, T>,
                    M: MemoryAccess,
                    S: SideEffectAccess,
                    T: TimingAccess,
                {
                    match self {
                        #(#execute_match_arms,)*
                        _ => unreachable!("Encountered unknown opcode"),
                    }
                }
            }

        };

        let enum_def = quote! {
            #[derive(Debug)]
            pub enum OpCode {
                #(#enum_variants)*
            }
        };

        let mut print_match_arms = Vec::new();
        for entry in &self.entries {
            let name = &entry.name;

            let print_lines = entry.fields.iter().map(|field| match field {
                OpcodeField::Normal(ident) => {
                    let label = ident.to_string();
                    quote! {
                        println!("{}  {}:", prefix, #label);
                        print_node_tree(nodes, op.#ident, indent + 2);
                    }
                }
                OpcodeField::Rest(ident) => {
                    let label = ident.to_string();
                    quote! {
                        println!("{}  {}:", prefix, #label);
                        for &child in &op.#ident {
                            print_node_tree(nodes, child, indent + 2);
                        }
                    }
                }
            });

            print_match_arms.push(quote! {
                OpCode::#name(op) => {
                    println!("{}{}:", prefix, stringify!(#name));
                    #(#print_lines)*
                }
            });
        }

        let print_fn = quote! {
            pub fn print_node_tree(nodes: &[ResolvedNode], index: usize, indent: usize) {
                let prefix = "  ".repeat(indent);
                match &nodes[index] {
                    ResolvedNode::Value(value) => {
                        println!("{}{value}", prefix);
                    },
                    ResolvedNode::OpCode(opcode) => {
                        match opcode {
                            #(#print_match_arms,)*
                        }
                    }
                }
            }
        };

        quote! {
            #(#struct_defs)*

            #enum_def

            #executor_impl

            #print_fn

            #[derive(Debug, Hash, PartialEq, Eq)]
            pub struct UnknownOpCodeError(pub String);

            pub fn resolve_opcode(name: String, args: Vec<usize>) -> Result<OpCode, UnknownOpCodeError> {
                Ok(match name.as_str() {
                    #(#match_arms,)*
                    _ => {
                        return Err(UnknownOpCodeError(name));
                    },
                })
            }
        }
    }
}

pub fn opcode_registry(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as OpcodeRegistryInput);
    input.expand().into()
}
