extern crate proc_macro;

use crate::utils::parse_;
use proc_macro2::TokenStream;
use quote2::*;
use syn::{
    Index,
    parse::{Parse, ParseStream, Result},
    *,
};

pub struct OpcodeRegistryInput {
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

impl OpcodeField {
    fn is_rest(&self) -> bool {
        matches!(self, OpcodeField::Rest(_))
    }
}

impl Parse for OpcodeRegistryInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut entries = Vec::new();
        while !input.is_empty() {
            entries.push(parse_::<_, Token![,]>(input)?);
        }
        Ok(Self { entries })
    }
}

impl Parse for OpcodeEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let mut fields = Vec::new();

        let c;
        braced!(c in input);
        while !c.is_empty() {
            fields.push(parse_::<_, Token![,]>(&c)?);
        }

        Ok(Self { name, fields })
    }
}

impl Parse for OpcodeField {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![..]) {
            input.parse::<Token![..]>()?;
            Ok(OpcodeField::Rest(input.parse()?))
        } else {
            Ok(OpcodeField::Normal(input.parse()?))
        }
    }
}

fn validate_input(input: &OpcodeRegistryInput) {
    for entry in &input.entries {
        let name = &entry.name;
        // Ensure there is at most one rest `..` pattern per entry.
        if entry
            .fields
            .iter()
            .filter(|f| matches!(f, OpcodeField::Rest(_)))
            .count()
            > 1
        {
            panic!("Cannot have more than one rest (`..`) pattern in `{name}`");
        }
    }
}

pub fn opcode_registry(t: &mut TokenStream, input: OpcodeRegistryInput) {
    validate_input(&input);

    for entry in &input.entries {
        let name = &entry.name;
        // 2. Generate the struct definition.
        let struct_fields = quote(|t| {
            for field in &entry.fields {
                match field {
                    OpcodeField::Normal(ident) => {
                        quote!(t, { pub #ident: usize, });
                    }
                    OpcodeField::Rest(ident) => {
                        quote!(t, { pub #ident: Vec<usize>, });
                    }
                }
            }
        });

        quote!(t, {
            #[derive(Debug, PartialEq)]
            pub struct #name {
                #struct_fields
            }
        });
    }

    let match_arms = quote(|t| {
        for entry in &input.entries {
            let name = &entry.name;
            let rest_position = entry.fields.iter().position(OpcodeField::is_rest);

            let (leading_len, trailing_len) = match rest_position {
                None => (entry.fields.len(), 0),
                Some(pos) => {
                    let leading = pos;
                    let trailing = entry.fields.len() - 1 - pos;
                    (leading, trailing)
                }
            };

            // 3. CORRECTED LOGIC: Build assignments using direct indexing into partitioned vectors.
            let assignments = quote(|t| {
                let mut leading_idx = 0;
                let mut trailing_idx = 0;

                for (i, field) in entry.fields.iter().enumerate() {
                    match field {
                        OpcodeField::Rest(ident) => {
                            quote!(t, { #ident: rest_args, });
                        }
                        OpcodeField::Normal(ident) => {
                            if rest_position.is_none_or(|pos| i < pos) {
                                // It's a leading field. Use an index into `leading_args`.
                                let idx = Index::from(leading_idx);
                                leading_idx += 1;
                                quote!(t, { #ident: leading_args[#idx], });
                            } else {
                                // It's a trailing field. Use an index into `trailing_args`.
                                let idx = Index::from(trailing_idx);
                                trailing_idx += 1;
                                quote!(t, { #ident: trailing_args[#idx], });
                            }
                        }
                    }
                }
            });

            // 4. Generate the match arm with the CORRECT runtime logic.
            quote!(t, {
                stringify!(#name) => {
                    let mut args_vec = args;
                    let total_len = args_vec.len();
                    let expected_min_len = #leading_len + #trailing_len;

                    if total_len < expected_min_len {
                        return Err(errors::InvalidArgumentCount {
                            function: stringify!(#name),
                            expected_min: expected_min_len,
                            actual: total_len,
                        }.into());
                    }

                    // Partition the runtime vector into three distinct Vecs.
                    let mut trailing_args = args_vec.split_off(total_len - #trailing_len);
                    let mut rest_args = args_vec.split_off(#leading_len);
                    let leading_args = args_vec; // The remainder is the leading args

                    // Construct the struct using the index-based assignments.
                    // Because `usize` is `Copy`, direct indexing works perfectly.
                    OpCode::#name(#name {
                        #assignments
                    })
                }
            });
        }
    });

    let enum_variants = quote(|t| {
        for entry in &input.entries {
            let name = &entry.name;
            quote!(t, { #name(#name), });
        }
    });

    let execute_match_arms = quote(|t| {
        for entry in &input.entries {
            let name = &entry.name;
            quote!(t, {
                OpCode::#name(executable) => executable.execute(executor),
            });
        }
    });

    let print_match_arms = quote(|t| {
        for entry in &input.entries {
            let name = &entry.name;

            let print_lines = quote(|t| {
                for field in &entry.fields {
                    match field {
                        OpcodeField::Normal(ident) => {
                            let label = ident.to_string();
                            quote!(t, {
                                println!("{}  {}:", prefix, #label);
                                print_node_tree(nodes, op.#ident, indent + 2);
                            });
                        }
                        OpcodeField::Rest(ident) => {
                            let label = ident.to_string();
                            quote!(t, {
                                println!("{}  {}:", prefix, #label);
                                for &child in &op.#ident {
                                    print_node_tree(nodes, child, indent + 2);
                                }
                            });
                        }
                    }
                }
            });

            quote!(t, {
                OpCode::#name(op) => {
                    println!("{}{}:", prefix, stringify!(#name));
                    #print_lines
                }
            });
        }
    });

    quote!(t, {
        #[derive(Debug)]
        pub enum OpCode {
            #enum_variants
        }

        impl<E, M, S, T> Executable<E, M, S, T> for OpCode {
            fn execute(&self, executor: E) -> (E, f64)
            where
                E: Executor<M, S, T>,
                M: MemoryAccess,
                S: SideEffectAccess,
                T: TimingAccess,
            {
                match self {
                    #execute_match_arms
                    _ => unreachable!("Encountered unknown opcode"),
                }
            }
        }

        pub fn print_node_tree(nodes: &[ResolvedNode], index: usize, indent: usize) {
            let prefix = "  ".repeat(indent);
            match &nodes[index] {
                ResolvedNode::Value(value) => {
                    println!("{}{value}", prefix);
                },
                ResolvedNode::OpCode(opcode) => {
                    match opcode {
                        #print_match_arms
                    }
                }
            }
        }

        pub fn resolve_opcode(name: String, args: Vec<usize>) -> Result<OpCode> {
            Ok(match name.as_str() {
                #match_arms
                _ => {
                    return Err(errors::UnknownOpCode(name).into());
                },
            })
        }
    });
}
