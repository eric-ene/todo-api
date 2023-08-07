use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct User {
    pub name: String,
    pub uuid: String,
    pub key: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct UserResult {
    pub uuid: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Entry {
    pub id: u16,
    pub content: String,
    pub priority: u8,
    pub owners: Vec<String>,
    pub group: u16,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Group {
    pub id: u8,
    pub users: Vec<String>,
    pub name: String,
}

pub struct Database {
    pub users: Collection<User>,
    pub entries: Collection<Entry>,
    pub groups: Collection<Group>,
}

impl std::fmt::Display for User { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result: String = "".to_string();

        result += "User {\n";
        result += format!("    {},\n", self.name).as_str();
        result += format!("    {},\n", self.uuid).as_str();
        result += format!("    {}\n", self.key).as_str();
        result += "}";

        return write!(f, "{}", result);
    }  
} 