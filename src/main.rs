extern crate actix_web;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;
use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use config::Config;

#[derive(Debug, Deserialize)]
struct Settings {
  port: i16,
  bind: String,
  database: Database,
}

#[derive(Debug, Deserialize)]
struct Database {
    user: String,
    password: String,
    bind: String,
    port: i16,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Transect {
    id: String,
    start_date: i32,
    end_date: i32,
    start_lat: f32,
    start_lon: f32,
    end_lat: f32,
    end_lon: f32,
    vessel_id: String,
    bearing: i32,
    observer1_id: String,
    observer2_id: Option<String>
}

#[get("transect/")]
async fn all_transects(db_pool: web::Data<mysql::Pool>) -> Result<web::Json<Vec<Transect>>> {
    let t = Transect{
        id: String::from("3kfiefjslkdiefjslkfj"),
        start_date: 392093,
        end_date: 29309,
        start_lat: 239.90,
        start_lon: -239.90,
        end_lat: 903.09,
        end_lon: 390.90,
        vessel_id: String::from("lsdfeijefl"),
        bearing: 90,
        observer1_id: String::from("fiekfisl"),
        observer2_id: Some(String::from("slkefisl"))
    };
    Ok(web::Json(Vec::from([t])))
}

#[get("transect/{id}")]
async fn one_transect(path: web::Path<String>, db_pool: web::Data<mysql::Pool>) -> Result<web::Json<Transect>> {
    let id = path.into_inner();
    let t = Transect{
        id: id,
        start_date: 392093,
        end_date: 29309,
        start_lat: 239.90,
        start_lon: -239.90,
        end_lat: 903.09,
        end_lon: 390.90,
        vessel_id: String::from("lsdfeijefl"),
        bearing: 90,
        observer1_id: String::from("fiekfisl"),
        observer2_id: Some(String::from("slkefisl"))
    };
    Ok(web::Json(t))
}

#[post("transect/")]
async fn upsert_transect(transect: web::Json<Transect>, db_pool: web::Data<mysql::Pool>) -> impl Responder {
    format!("Saving transect data for {:?}", transect)
}

fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("config/config"))
        .build()
        .unwrap()
        .try_deserialize::<Settings>()
        .unwrap();

    let database_connection_string = format!("mysql://{}:{}@{}:{}/{}", 
        config.database.user, config.database.password, config.database.bind, 
        config.database.port, config.database.name);
    println!("database_connection_string: {}", database_connection_string);

    let pool = mysql::Pool::new(database_connection_string).unwrap();

    pool.prep_exec(r"CREATE TABLE if not exists payment (
        customer_id int not null,
        account_name text
    )", ()).unwrap();

    start_server(config, pool).unwrap();
}

#[actix_web::main]
async fn start_server(config: Settings, pool: mysql::Pool) -> std::io::Result<()> {
    let connection_string = format!("{}:{}", config.bind, config.port);
    println!("connection_string: {}", connection_string);

    HttpServer::new(move || 
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(all_transects)
                .service(one_transect)
                .service(upsert_transect)
        )
        .bind(connection_string)?
        .run()
        .await
}

