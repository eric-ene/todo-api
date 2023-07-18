use actix_web::{ HttpRequest, Result, get, web };

use crate::utils;

#[get("/")]
pub async fn root(req: HttpRequest) -> Result<String> {
  utils::print_req_info(req);
  return Ok("Hello, world!".to_string());
}

#[get("/all")]
pub async fn all(req: HttpRequest) -> Result<String> {
  utils::print_req_info(req);

  let data: utils::Data = {
    let raw_text = std::fs::read_to_string("data/data.json").unwrap();

    serde_json::from_str::<utils::Data>(&raw_text).unwrap()
  };

  return Ok(format!("{}", serde_json::to_string_pretty(&data).unwrap()));
}

#[get("/user/{id}")]
pub async fn by_id(req: HttpRequest, info: web::Path<utils::Info>) -> Result<String> {
  utils::print_req_info(req);

  let data: utils::Data = {
    let raw_text = std::fs::read_to_string("data/data.json").unwrap();

    serde_json::from_str::<utils::Data>(&raw_text).unwrap()
  };

  let user = utils::find_in_json(data.clone(), &info.id);

  if user.is_some() {
    return Ok(format!("{}", serde_json::to_string_pretty(&user).unwrap()));
  }

  return Ok("{\"id\": \"none\",\"list\": [\"none\"]}".to_string());
}