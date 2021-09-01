extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use syn::spanned::Spanned;
use proc_macro2::{Ident, Span};

#[proc_macro_derive(Insert)]
pub fn insert_derive(input: TokenStream) -> TokenStream {
    let mut insert_tokens = vec![];
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;
    match parsed_input.data {
        Data::Struct(s) => {
            if let Fields::Named(name_fields) = s.fields {
                let a = name_fields.named;
                for field in a {
                    let field = field.ident.as_ref().unwrap();
                    let insert_token = quote! {
                        .add(stringify!(#field), vec![self.#field])
                    };
                    insert_tokens.push(insert_token);
                }
            }
        }
        other => { panic!("ToVec is not yet implemented for: {:?} ", other) }
    }
    let tokens = quote! {
        #[async_trait]
        impl Insert for #struct_name {
            async fn insert(&self, table_name: &str) -> Result<(), Box<dyn std::error::Error>> {
                let mut connection = CLICKHOUSE.connection().await?;
                let block = Block::new(table_name)
                #(#insert_tokens)*
                ;
                let insert = connection.insert(&block).await?;
                insert.commit().await?;
                drop(insert);
                Ok(())
            }
        }
    };
    eprintln!("{:#?}", tokens.to_string());
    proc_macro::TokenStream::from(tokens)
}

#[proc_macro_derive(Query)]
pub fn query_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;
    let tokens = quote! {
        #[async_trait]
        impl Query<#struct_name> for #struct_name {
            async fn query(&self, sql: &str) -> Result<Vec<#struct_name>, Box<dyn std::error::Error>> {
                let mut connection = CLICKHOUSE.connection().await?;
                let mut query = connection.query(sql).await?;
                let mut rows = Vec::new();
                while let Some(block) = query.next().await? {
                    for row in block.iter::<#struct_name>() {
                        rows.push(row);
                    }
                }
                Ok(rows)
            }
        }
    };
    eprintln!("{:#?}", tokens.to_string());
    proc_macro::TokenStream::from(tokens)
}