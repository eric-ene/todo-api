use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use mongodb::{options::ClientOptions, Client, bson::doc};

mod req;
mod utils;
mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = database_setup("mongodb://localhost:27017").await;

    return server_setup(8080, db).await;
}

async fn server_setup(port: u16, db: web::Data<database::Database>) -> std::io::Result<()> {
    println!("> Starting api server...");

    let server = HttpServer::new(move || {
        let cors: Cors = Cors::default()
            .allowed_origin("https://ericalexander.ca")
            .allowed_origin("http://192.168.0.177:4200")
            .allow_any_method()
            .allow_any_header();

        return App::new()
            .app_data(db.clone())
            .wrap(cors)
            .service(req::root)
            .service(req::all)
            .service(req::by_id)
            // .service(req::remove)
            .service(req::remove_owner)
            .service(req::new_user)
            .service(req::user_by_uuid)
            .service(req::group_by_id)
            .service(req::group_by_user)
            .service(req::edit_entry);
    });

    println!("> Now listening on 0.0.0.0:{}...", port);

    return server
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await;
}

async fn database_setup(address: &str) -> web::Data<database::Database> {
    let uri = address;
    let client_options = ClientOptions::parse(uri).await.unwrap();

    let client = Client::with_options(client_options).unwrap();
    let db = client.database("todo");

    db.run_command(doc! {"ping": 1}, None)
        .await.unwrap();

    println!("Pinged your deployment! Connection successful.");

    let users = db.collection::<database::User>("users");
    let entries = db.collection::<database::Entry>("entries");
    let groups = db.collection::<database::Group>("groups");

    let db_object = web::Data::new(database::Database{
            users: users,
            entries: entries,
            groups: groups,
    });
        
    return db_object;
}
