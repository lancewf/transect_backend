extern crate actix_web;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;
use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use config::Config;
extern crate chrono;

use chrono::prelude::*;

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
    start_date: i64,
    end_date: i64,
    start_lat: f32,
    start_lon: f32,
    end_lat: f32,
    end_lon: f32,
    vessel_id: String,
    bearing: i32,
    observer1_id: String,
    observer2_id: Option<String>,
    observations: Vec<Observation>
}

#[derive(Debug, Deserialize, Serialize)]
struct Observation {
    id: String,
    transect_id: String,
    obs_type: String,
    date: i64,
    lat: f32,
    lon: f32,
    bearing: Option<i64>,
    count: Option<i64>,
    distance_km: Option<f32>,
    group_type: Option<String>,
    beaufort_type: Option<String>,
    weather_type: Option<String>,
}

#[get("transect/")]
async fn all_transects(db_pool: web::Data<mysql::Pool>) -> Result<web::Json<Vec<Transect>>> {
    let query = "SELECT id, start_date, end_date, bearing, start_lat, \
        start_lon, end_lat, end_lon, vessel_id, observer1_id, observer2_id from transect";
    let rows = db_pool.prep_exec(query, ()).map(|result| {
            result.map(|x| x.unwrap())}).unwrap();

    let transects: Vec<Transect> = rows.map(|row| {
        let (id, start_date, end_date, bearing, 
            start_lat, start_lon, end_lat, end_lon, vessel_id, 
            observer1_id, observer2_id) = 
            mysql::from_row::<(String, NaiveDateTime, NaiveDateTime, i32, f32, f32, f32, f32, String, String, Option<String>)>(row);

        let observations = get_observations(id.clone(), &db_pool);

        Transect{
            id,
            start_date: start_date.timestamp(),
            end_date: end_date.timestamp(),
            start_lat,
            start_lon,
            end_lat,
            end_lon,
            vessel_id,
            bearing,
            observer1_id,
            observer2_id,
            observations
        }
    }).collect();

    Ok(web::Json(transects))
}


#[get("transect/{id}")]
async fn one_transect(path: web::Path<String>, db_pool: web::Data<mysql::Pool>) -> Result<web::Json<Option<Transect>>> {
    let transect_id = path.into_inner();
    let query = "SELECT id, start_date, end_date, bearing, start_lat, start_lon, end_lat, \
        end_lon, vessel_id, observer1_id, observer2_id from transect WHERE id = :id";

    let params = params! {"id" => &transect_id};
    let mut rows = db_pool.prep_exec(query, params).map(
        |result| result.map(|x| x.unwrap())).unwrap();

    let transect: Option<Transect> = rows.next().map(|row| {
        let (id, start_date, end_date, bearing, 
            start_lat, start_lon, end_lat, end_lon, vessel_id, 
            observer1_id, observer2_id) = 
            mysql::from_row::<(String, NaiveDateTime, NaiveDateTime, i32, f32, f32, f32, f32, String, String, Option<String>)>(row);

        let observations = get_observations(transect_id, &db_pool);

        Transect{
            id,
            start_date: start_date.timestamp(),
            end_date: end_date.timestamp(),
            start_lat,
            start_lon,
            end_lat,
            end_lon,
            vessel_id,
            bearing,
            observer1_id,
            observer2_id,
            observations
        }
    });

    Ok(web::Json(transect))
}

fn get_observations(transect_id: String, db_pool: &web::Data<mysql::Pool>) -> Vec<Observation> {
    let query = "SELECT id, obs_type, date, bearing, count, lat, lon, distance_km, group_type, \
        beaufort_type, weather_type from observation WHERE transect_id = :transect_id";

    let rows = db_pool.prep_exec(query, params!{"transect_id" => transect_id.clone()}).map(|result| 
            result.map(|x| x.unwrap())).unwrap();

    rows.map(|row| {
        let (id, obs_type, date, bearing, count, lat, lon, distance_km, 
            group_type, beaufort_type, weather_type) = 
            mysql::from_row::<(String, String, NaiveDateTime, Option<i64>, Option<i64>, f32, f32, Option<f32>, Option<String>, Option<String>, Option<String>)>(row);

        Observation{
            id,
            transect_id: transect_id.clone(),
            obs_type,
            date: date.timestamp(),
            bearing,
            count,
            lat,
            lon,
            distance_km,
            group_type,
            beaufort_type,
            weather_type
        }
    }).collect()
}

fn format_date(epoch_utc_date: i64) -> String {
    // Create a NaiveDateTime from the timestamp
    let naive = NaiveDateTime::from_timestamp(epoch_utc_date, 0);
    
    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    
    // Format the datetime how you want
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
 
    format!("{}", newdate)
}

#[post("transect/")]
async fn upsert_transect(transect: web::Json<Transect>, db_pool: web::Data<mysql::Pool>) -> impl Responder {
    let start_date = format_date(transect.start_date);
    let end_date = format_date(transect.end_date);
    
    let _ : Vec<mysql::QueryResult> = transect.observations.iter().map(
        |observation| {
        let date = format_date(observation.date);

        db_pool.prep_exec(r"INSERT INTO observation 
            ( id, transect_id, obs_type, date, bearing, count, lat, lon, distance_km, group_type, beaufort_type, weather_type) VALUES 
            (:id, :transect_id, :obs_type, :date, :bearing, :count, :lat, :lon, :distance_km, :group_type, :beaufort_type, :weather_type)
            ON DUPLICATE KEY UPDATE transect_id=:transect_id, obs_type=:obs_type, date=:date, bearing=:bearing, count=:count,
            lat=:lat, lon=:lon, distance_km=:distance_km, group_type=:group_type, beaufort_type=:beaufort_type, weather_type=:weather_type",
            params!{"id" => &observation.id, "transect_id" => &observation.transect_id, 
            "obs_type" => &observation.obs_type, "date" => date, "bearing" => observation.bearing,
            "count" => observation.count, "lat" => observation.lat, "lon" => observation.lon, 
            "distance_km" => observation.distance_km, "group_type" => &observation.group_type, 
            "beaufort_type" => &observation.beaufort_type, "weather_type" => &observation.weather_type}).unwrap()
    }).collect();


    db_pool.prep_exec(r"INSERT INTO transect 
        ( id, bearing, start_date, end_date, start_lat, start_lon, end_lat, end_lon, vessel_id, observer1_id, observer2_id) VALUES 
        (:id, :bearing, :start_date, :end_date, :start_lat, :start_lon, :end_lat, :end_lon, :vessel_id, :observer1_id, :observer2_id)
        ON DUPLICATE KEY UPDATE bearing=:bearing, start_date=:start_date, end_date=:end_date, start_lat=:start_lat, start_lon=:start_lon, end_lat=:end_lat, end_lon=:end_lon,
        vessel_id=:vessel_id, observer1_id=:observer1_id, observer2_id=:observer2_id",
        params!{"id" => &transect.id, "bearing" => transect.bearing, "start_date" => start_date, "end_date" => end_date, "start_lat" => transect.start_lat, 
            "start_lon" => transect.start_lon, "end_lat" => transect.end_lat, "end_lon" => transect.end_lon, "vessel_id" => &transect.vessel_id, 
            "observer1_id" => &transect.observer1_id, "observer2_id" => &transect.observer2_id}).unwrap();

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

