use actix_cors::Cors;
use actix_web::{ HttpServer, App };

mod gets;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let port: u16 = 25565;

  println!("> Starting api server...");

  let server = HttpServer::new(
    || {
      let cors: Cors = Cors::default()
        .allowed_origin("http://localhost:4200");

      return App::new()
        .wrap(cors)
        .service(gets::root)
        .service(gets::all)
        .service(gets::by_id)
  });

  println!("> Now listening on port {}...", port);
  return server.bind(("127.0.0.1", port))?.run().await;
}

