use std::collections::{HashMap as Map, HashSet as Set};

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote2::*;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream, Result},
    *,
};

pub struct MemoryAccessInput {
    blocks: Vec<Ident>,
    mutability_by_stage: Map<Ident, Set<Ident>>,
}

impl Parse for MemoryAccessInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut blocks = Vec::new();

        let c;
        bracketed!(c in input);
        while !c.is_empty() {
            blocks.push(parse_ident::<Token![,]>(&c)?);
        }

        let mut mutability_by_stage = Map::new();

        while !input.is_empty() {
            let stage_name = parse_ident::<Token![:]>(input)?;

            let c;
            bracketed!(c in input);
            while !c.is_empty() {
                let block_name = parse_ident::<Token![,]>(&c)?;

                mutability_by_stage
                    .entry(stage_name.clone())
                    .and_modify(|items: &mut Set<_>| {
                        items.insert(block_name.clone());
                    })
                    .or_insert(Set::from([block_name]));
            }
        }

        Ok(MemoryAccessInput {
            blocks,
            mutability_by_stage,
        })
    }
}

fn parse_ident<Sep: Parse>(input: &ParseBuffer<'_>) -> Result<Ident> {
    let name = input.parse()?;
    // Optional comma after individual item
    let _ = input.parse::<Sep>();
    Ok(name)
}

pub fn generate_memory_access(t: &mut TokenStream, input: MemoryAccessInput) {
    let MemoryAccessInput {
        blocks,
        mutability_by_stage,
    } = input;

    for (stage_name, mutable_blocks) in mutability_by_stage.clone() {
        let struct_name = format_ident!("{}MemoryAccess", stage_name);
        let fields = quote(|t| {
            for block_name in &blocks {
                let snake_case_block_name = snake_case(block_name);
                let field_ty = quote(|t| {
                    let is_mut = mutable_blocks.contains(block_name);
                    if is_mut {
                        quote!(t, { ResMut<'w, #block_name> });
                    } else {
                        quote!(t, { Res<'w, #block_name> });
                    }
                });

                quote!(t, {
                    pub #snake_case_block_name: #field_ty,
                });
            }
        });

        let block_match_arms = quote(|t| {
            for block_name in &blocks {
                let snake_case_block_name = snake_case(block_name);
                quote!(t, {
                    #block_name::ID => self.#snake_case_block_name.read(index),
                });
            }
        });

        let block_match_arms_mut = quote(|t| {
            for block_name in blocks
                .iter()
                .filter(|block_name| mutable_blocks.contains(block_name))
            {
                let snake_case = snake_case(block_name);
                quote!(t, {
                    #block_name::ID => self.#snake_case.write(index, value),
                });
            }
        });

        let struct_name_string = struct_name.to_string();
        quote!(t, {
            #[derive(::bevy::ecs::system::SystemParam)]
            pub struct #struct_name<'w> {
                #fields
            }

            impl<'w> MemoryAccess for #struct_name<'w> {
                fn read(&self, current_entity: EntityId, block_id: u16, index: usize) -> Option<f64> {
                    let (block_id, index) = resolve_entity_index(block_id, current_entity, index);
                    Some(
                        match block_id {
                            #block_match_arms
                            _ => unreachable!("Unknown block ID: {}", block_id),
                        }.unwrap_or_default()
                    )
                }

                fn write(&mut self, current_entity: EntityId, block_id: u16, index: usize, value: f64) {
                    let (block_id, index) = resolve_entity_index(block_id, current_entity, index);
                    match block_id {
                        #block_match_arms_mut
                        _ => unreachable!("Unknown or immutable block ID: {}, {}", block_id, #struct_name_string),
                    }
                }
            }
        });
    }
}

fn snake_case(block_name: &Ident) -> Ident {
    Ident::new(&block_name.to_string().to_snake_case(), block_name.span())
}
