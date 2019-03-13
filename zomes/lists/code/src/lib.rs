
#[macro_use]
extern crate hdk;
    
#![feature(try_from)]
use std::convert::TryFrom;
#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_core_types::{
        hash::HashString,
        error::HolochainError,
        dna::entry_types::Sharing,
        json::JsonString,
        cas::content::Address,
        entry::{AppEntryValue, Entry},
    }
};
 
define_zome! {
    entries: [
        entry!(
            name: "list",
            description: "",
            sharing: Sharing::Public,
            native_type: List,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |list: List, _ctx: hdk::ValidationData| {
                Ok(())
            },
            links: [
                to!(
                    "listItem",
                    tag: "items",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]
        ),
        entry!(
            name: "listItem",
            description: "",
            sharing: Sharing::Public,
            native_type: ListItem,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |list_item: ListItem, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    ]
 
    genesis: || {
        Ok(())
    }
 
    functions: [
    ]
 
    traits: {
    }
}



#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct List {
    name: String
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct ListItem {
    text: String,
    completed: bool
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct GetListResponse {
    name: String,
    items: Vec<ListItem>
}s