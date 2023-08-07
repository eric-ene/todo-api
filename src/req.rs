use actix_web::{get, post, web, HttpRequest, Result};
use mongodb::bson::doc;

use crate::{utils, database};

#[get("/")]
pub async fn root(req: HttpRequest) -> Result<String> {
    utils::print_req_info(req);
    return Ok("Hello, world!".to_string());
}

#[get("/all")]
pub async fn all(req: HttpRequest, data: web::Data<database::Database>) -> Result<String> {
    utils::print_req_info(req);
    let mut vec: Vec<database::Entry> = vec![];

    let entries = &data.entries;

    let mut cursor = entries.find(None, None).await.unwrap();

    while cursor.advance().await.unwrap() {
        vec.push(cursor.deserialize_current().unwrap());
    } 

    return Ok(format!("{}", serde_json::to_string_pretty(&vec).unwrap()));
}

#[get("/groups/{uuid}")]
pub async fn group_by_user(req: HttpRequest, info: web::Path<utils::InfoUser>, data: web::Data<database::Database>) -> Result<String> {
    utils::print_req_info(req);
    let mut vec: Vec<database::Group> = vec![];

    let groups = &data.groups;

    let mut cursor = groups.find(doc!{ "users": &info.uuid }, None).await.unwrap();

    while cursor.advance().await.unwrap() {
        vec.push(cursor.deserialize_current().unwrap());
    } 

    return Ok(format!("{}", serde_json::to_string_pretty(&vec).unwrap()));
}

#[get("/id/{id}")]
pub async fn by_id(req: HttpRequest, info: web::Path<utils::Info>, data: web::Data<database::Database>) -> Result<String> {
    utils::print_req_info(req);

    let entries = &data.entries;
    let result = entries.find_one(doc!{ "id": info.id as u32 }, None)
        .await
        .unwrap_or_default()
        .unwrap_or_default();

    return Ok(format!("{}", serde_json::to_string_pretty(&result).unwrap()));
}

#[get("/group/{id}")]
pub async fn group_by_id(req: HttpRequest, info: web::Path<utils::Info>, data: web::Data<database::Database>) -> Result<String> {
    utils::print_req_info(req);

    let groups = &data.groups;
    let result = groups.find_one(doc!{ "id": info.id as u32 }, None)
        .await
        .unwrap_or_default()
        .unwrap_or_default();

    return Ok(format!("{}", serde_json::to_string_pretty(&result).unwrap()));
}

#[get("/user/{uuid}")]
pub async fn user_by_uuid(
    req: HttpRequest, 
    info: web::Path<utils::InfoUser>, 
    data: web::Data<database::Database>,
) -> Result<String> {
    utils::print_req_info(req);

    let query = doc! {
        "uuid": &info.uuid
    };

    let users = &data.users;
    let result = users.find_one(query, None)
        .await
        .unwrap_or_default()
        .unwrap_or_default();

    let result_name_only = database::UserResult {
        uuid: info.uuid.clone(),
        name: result.name,
    };

    return Ok(format!("{}", serde_json::to_string_pretty(&result_name_only).unwrap()));
}

// #[post("/remove-entry")]
// pub async fn remove(req: HttpRequest, post: web::Json<utils::RmPost>) -> Result<String> {
//     utils::print_req_info(req);



//     return Ok("a".to_string());
// }

#[post("/remove-owner")]
pub async fn remove_owner(
    req: HttpRequest, 
    post: web::Json<utils::RmOwner>,
    data: web::Data<database::Database>,
) -> Result<String> {
    utils::print_req_info(req);

    let query = doc! {
        "id": post.0.entry_id as u32
    };

    let entries = &data.entries;
    let mut entry = entries.find_one(query.clone(), None).await.unwrap().unwrap();
    entry.owners.remove(post.0.owner_index as usize);

    let status = entries.replace_one(query, entry, None).await;

    let str = if status.is_ok() { 
        format!("Updated entry with id {}.", &post.0.entry_id) 
    } else { 
        format!("Cannot update entry with id {}.", &post.0.entry_id)
    };

    let status = utils::Response {
        status_type: "add".to_string(),
        status_value: str,
    };

    let json = serde_json::to_string_pretty(&status).unwrap();

    return Ok(json);
}


#[post("/new-user")]
pub async fn new_user(
    req: HttpRequest, 
    post: web::Json<database::User>, 
    data: web::Data<database::Database>,
) -> Result<String> {
    utils::print_req_info(req);

    let status = data.
        users.insert_one(&post.0, None).await;

    if status.is_ok() { println!("Created new user with uuid {}.", post.0.uuid); }
    else { println!("User \"{}\" already exists!", post.0.uuid) }

    
    return Ok("a".to_string())
}

#[post("/edit-entry")]
pub async fn edit_entry(
    req: HttpRequest,
    post: web::Json<database::Entry>,
    data: web::Data<database::Database>,
) -> Result<String> {
    utils::print_req_info(req);

    let entries = &data.entries;
    let query = doc!{ "id": post.0.id as u32 };

    let status = entries.replace_one(query, &post.0, None).await;

    let str = if status.is_ok() { 
        format!("Updated entry with id {}.", &post.0.id) 
    } else { 
        format!("Cannot update entry with id {}.", &post.0.id)
    };

    let status = utils::Response {
        status_type: "add".to_string(),
        status_value: str,
    };

    let json = serde_json::to_string_pretty(&status).unwrap();

    println!("{}", json);

    return Ok(json);
}