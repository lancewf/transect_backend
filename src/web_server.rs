use super::build_config;
use super::model;
use super::data_store;
use actix_web::{get, post, web, App, HttpServer, Responder, Result, HttpRequest, error::ErrorForbidden};

#[get("transect/")]
async fn all_transects(request: HttpRequest, data: web::Data<(mysql::Pool, build_config::Settings)>) -> Result<web::Json<Vec<model::Transect>>> {
    let db_pool = &data.0;
    let config = &data.1;
    if !request_valid(request, config) {
        Err(ErrorForbidden("Not allowed"))
    } else {
        let transects: Vec<model::Transect> = data_store::get_all_transects(&db_pool);

        Ok(web::Json(transects))
    }
}

#[get("observer/")]
async fn all_observers(request: HttpRequest, data: web::Data<(mysql::Pool, build_config::Settings)>) -> Result<web::Json<Vec<model::Observer>>> {
    let db_pool = &data.0;
    let config = &data.1;
    if !request_valid(request, config) {
        Err(ErrorForbidden("Not allowed"))
    } else {
        let observers: Vec<model::Observer> = data_store::get_all_observers(&db_pool);

        Ok(web::Json(observers))
    }
}

#[get("vessel/")]
async fn all_vessel(request: HttpRequest, data: web::Data<(mysql::Pool, build_config::Settings)>) -> Result<web::Json<Vec<model::Vessel>>> {
    let db_pool = &data.0;
    let config = &data.1;
    if !request_valid(request, config) {
        Err(ErrorForbidden("Not allowed"))
    } else {
        let vessels: Vec<model::Vessel> = data_store::get_all_vessels(&db_pool);

        Ok(web::Json(vessels))
    }
}

#[get("transect/{id}")]
async fn one_transect(request: HttpRequest, path: web::Path<String>, data: web::Data<(mysql::Pool, build_config::Settings)>) -> Result<web::Json<Option<model::Transect>>> {
    let db_pool = &data.0;
    let config = &data.1;
    if !request_valid(request, config) {
        Err(ErrorForbidden("Not allowed"))
    } else {
        let transect_id = path.into_inner();

        let transect: Option<model::Transect> = 
            data_store::get_transect_by_id(transect_id, &db_pool);

        Ok(web::Json(transect))
    }
}

#[post("transect/")]
async fn upsert_transect(request: HttpRequest, transect: web::Json<model::Transect>, data: web::Data<(mysql::Pool, build_config::Settings)>) -> impl Responder {
    let db_pool = &data.0;
    let config = &data.1;
    if !request_valid(request, config) {
        Err(ErrorForbidden("Not allowed"))
    } else {
        data_store::upsert_transect(&transect, &db_pool);

        Ok(format!("Saving transect data for {:?}", transect))
    }
}

fn request_valid(request: HttpRequest, config: &build_config::Settings) -> bool {
    if config.api_validation {
        let req_headers = request.headers();
        let key = req_headers.get("api-key");

        key.map(|h| h.to_str().unwrap().eq(&config.api_key)).unwrap_or(false)
    } else {
        true
    }
}

#[actix_web::main]
pub async fn start_server(config: build_config::Settings, pool: mysql::Pool) -> std::io::Result<()> {
    let connection_string = format!("{}:{}", config.bind, config.port);
    println!("connection_string: {}", connection_string);

    HttpServer::new(move || 
            App::new()
                .app_data(web::Data::new((pool.clone(), config.clone())))
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
