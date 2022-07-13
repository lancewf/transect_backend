use super::build_config;
use super::model;
use super::data_store;
use actix_web::{get, post, web, App, HttpServer, Responder, Result};

#[get("transect/")]
async fn all_transects(db_pool: web::Data<mysql::Pool>) -> Result<web::Json<Vec<model::Transect>>> {
    let transects: Vec<model::Transect> = data_store::get_all_transects(&db_pool);

    Ok(web::Json(transects))
}

#[get("observer/")]
async fn all_observers(db_pool: web::Data<mysql::Pool>) -> Result<web::Json<Vec<model::Observer>>> {
    let observers: Vec<model::Observer> = data_store::get_all_observers(&db_pool);

    Ok(web::Json(observers))
}

#[get("vessel/")]
async fn all_vessel(db_pool: web::Data<mysql::Pool>) -> Result<web::Json<Vec<model::Vessel>>> {
    let vessels: Vec<model::Vessel> = data_store::get_all_vessels(&db_pool);

    Ok(web::Json(vessels))
}

#[get("transect/{id}")]
async fn one_transect(path: web::Path<String>, db_pool: web::Data<mysql::Pool>) -> Result<web::Json<Option<model::Transect>>> {
    let transect_id = path.into_inner();

    let transect: Option<model::Transect> = 
        data_store::get_transect_by_id(transect_id, &db_pool);

    Ok(web::Json(transect))
}

#[post("transect/")]
async fn upsert_transect(transect: web::Json<model::Transect>, db_pool: web::Data<mysql::Pool>) -> impl Responder {
    data_store::upsert_transect(&transect, &db_pool);

    format!("Saving transect data for {:?}", transect)
}

#[actix_web::main]
pub async fn start_server(config: build_config::Settings, pool: mysql::Pool) -> std::io::Result<()> {
    let connection_string = format!("{}:{}", config.bind, config.port);
    println!("connection_string: {}", connection_string);

    HttpServer::new(move || 
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(all_transects)
                .service(one_transect)
                .service(upsert_transect)
                .service(all_vessel)
                .service(all_observers)
        )
        .bind(connection_string)?
        .run()
        .await
}
