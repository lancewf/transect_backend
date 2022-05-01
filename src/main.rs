extern crate actix_web;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;
use actix_web::{get, web, App, HttpServer, Responder};
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

#[get("/{customer_id}/{account_name}/index.html")]
async fn index(path: web::Path<(u32, String)>, db_pool: web::Data<mysql::Pool>) -> impl Responder {
    let (customer_id, account_name) = path.into_inner();

    db_pool.prep_exec("INSERT INTO payment (customer_id, account_name) VALUES (:customer_id, :account_name)", 
        mysql::params! {
                "customer_id" => customer_id,
                "account_name" => account_name.clone(),
        },
    ).unwrap();

    format!("Hello {}! id:{}", account_name, customer_id)
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
                .service(index)
        )
        .bind(connection_string)?
        .run()
        .await
}

