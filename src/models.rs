use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    id: u32,
    login: String,
}
#[derive(Deserialize, Debug)]
pub struct Gist {
    pub id: String,
    pub description: Option<String>,
    pub owner: Owner,
}
#[derive(Deserialize, Debug)]
pub struct Owner {
    pub login: String,
}