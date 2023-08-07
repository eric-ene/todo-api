use actix_web::HttpRequest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Info {
    pub id: u16,
}

#[derive(Deserialize)]
pub struct InfoUser {
    pub uuid: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Data {
    primary: Vec<Entry>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Entry {
    id: u16,
    content: String,
    priority: u8,
    owners: Vec<String>,
}

#[derive(Deserialize)]
pub struct RmPost {
    pub id: u16,
}

#[derive(Deserialize)]
pub struct RmOwner {
    pub entry_id: u16,
    pub owner_index: u16,
}


#[derive(Deserialize, Serialize)]
pub struct Response {
    pub status_type: String,
    pub status_value: String,
}

pub fn print_req_info(req: HttpRequest) {
    let origin = req.headers().get("origin");
    let origin_str = if !origin.is_none() {
        origin.unwrap().to_str().unwrap()
    } else {
        "none (likely direct from browser)"
    };

    println!("> request from: {}", origin_str);
    println!("           for: {}", req.uri().path());
}

