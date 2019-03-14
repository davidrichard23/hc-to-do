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
        create_list: {
            inputs: |list: List|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_list
        }
        add_item: {
            inputs: |list_item: ListItem, list_addr: HashString|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_add_item
        }
        get_list: {
            inputs: |list_addr: HashString|,
            outputs: |result: ZomeApiResult<GetListResponse>|,
            handler: handle_get_list
        }
    ]
    traits: {
        hc_public [create_list, add_item, get_list]
    }
}


fn handle_create_list(list: List) -> ZomeApiResult<Address> {
    let list_entry = Entry::App(
        "list".into(),
        list.into()
    );

	hdk::commit_entry(&list_entry)
}

fn handle_add_item(list_item: ListItem, list_addr: HashString) -> ZomeApiResult<Address> {
    let list_item_entry = Entry::App(
        "listItem".into(),
        list_item.into()
    );

	let item_addr = hdk::commit_entry(&list_item_entry)?;
	hdk::link_entries(&list_addr, &item_addr, "items")?;
	Ok(item_addr)
}

fn handle_get_list(list_addr: HashString) -> ZomeApiResult<GetListResponse> {

    let list = get_as_type::<List>(list_addr.clone())?;
    let list_items = hdk::get_links(&list_addr, "items")?.addresses()
        .iter()
        .map(|item_address| {
            get_as_type::<ListItem>(item_address.to_owned())
        })
        .filter_map(Result::ok)
        .collect::<Vec<ListItem>>();

    Ok(GetListResponse{
        name: list.name,
        items: list_items
    })
}

pub fn get_as_type<
    R: TryFrom<AppEntryValue>
> (address: HashString) -> ZomeApiResult<R> {
    let get_result = hdk::get_entry(&address)?;
    let entry = get_result.ok_or(ZomeApiError::Internal("No entry at this address".into()))?;
    match entry {
        Entry::App(_, entry_value) => {
            R::try_from(entry_value.to_owned())
                .map_err(|_| ZomeApiError::Internal(
                    "Could not convert get_links result to requested type".to_string())
                )
        },
        _ => Err(ZomeApiError::Internal(
            "get_links did not return an app entry".to_string())
        )
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
}