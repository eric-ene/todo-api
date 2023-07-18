use actix_web::HttpRequest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Info {
  pub id: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Data {
  users: Vec<User>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
  id: String,
  list: Vec<String>
}

pub fn print_req_info(req: HttpRequest) {
  let origin = req.headers().get("origin");
  let origin_str =  
    if !origin.is_none() { 
      origin.unwrap().to_str().unwrap() 
    } else { 
      "none" 
    };

  println!("> request from: {}", origin_str);
  println!("           for: {}", req.uri().path());
}

pub fn find_in_json(json: Data, id: &String) -> Option<User> {
  for entry in json.users {
    if &entry.id == id {
      return Some(entry);
    }
  }

  return None;
}
