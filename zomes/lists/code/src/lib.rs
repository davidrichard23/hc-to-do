
#[macro_use]
extern crate hdk;
 
define_zome! {
    entries: [
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
}