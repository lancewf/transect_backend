use super::build_config;
use super::model;
use chrono::prelude::*;
use geoutils::Location;

pub fn create_pool(config: &build_config::Settings) -> mysql::Pool {
    let database_connection_string = format!("mysql://{}:{}@{}:{}/{}", 
        config.database.user, config.database.password, config.database.bind, 
        config.database.port, config.database.name);
    println!("database_connection_string: {}", database_connection_string);

    mysql::Pool::new(database_connection_string).unwrap()
}

pub fn get_all_observers(db_pool: &mysql::Pool) -> Vec<model::Observer> {
    let query = "SELECT id, name from observer";

    let rows = db_pool.prep_exec(query,()).map(|result| 
            result.map(|x| x.unwrap())).unwrap();

    rows.map(|row| {
        let (id, name) = mysql::from_row::<(String, String)>(row);

        model::Observer{ id, name }
    }).collect()
}

pub fn get_all_vessels(db_pool: &mysql::Pool) -> Vec<model::Vessel> {
    let query = "SELECT id, name from vessel";

    let rows = db_pool.prep_exec(query,()).map(|result| 
            result.map(|x| x.unwrap())).unwrap();

    rows.map(|row| {
        let (id, name) = mysql::from_row::<(String, String)>(row);

        let transects: Vec<model::Transect> = get_transects_for_vessel(id.clone(), db_pool);

        let number_of_sightings = get_number_of_sightings_for_vessel(id.clone(), db_pool);
        let number_of_transects = transects.len();
        let total_duration_of_all_transects_secs: i64 = transects.iter().map(|transect| calculate_duration(transect)).sum();
        let number_of_animals: usize = get_sum_of_animals_for_vessel(id.clone(), db_pool);
        let total_distance_of_all_transects_km: f64 = transects.iter().map(|transect| calculate_distance_km(transect)).sum();

        model::Vessel{ id, name, number_of_transects, number_of_sightings, 
            total_duration_of_all_transects_secs, total_distance_of_all_transects_km, 
            number_of_animals}
    }).collect()
}

fn calculate_duration(transect: &model::Transect) -> i64 {
    transect.end_date - transect.start_date
}

fn calculate_distance_km(transect: &model::Transect) -> f64 {
    let start = Location::new(transect.start_lat, transect.start_lon);
    let end = Location::new(transect.end_lat, transect.end_lon);

    start.distance_to(&end).unwrap().meters() / 1000.0
}

fn get_number_of_sightings_for_vessel(vessel_id: String, db_pool: &mysql::Pool) -> usize {
    let query = "SELECT Count(*) FROM observation o INNER JOIN transect t ON o.transect_id=t.id WHERE t.vessel_id=:vessel_id AND o.obs_type='Sighting'";

    let rows = db_pool.prep_exec(query, params!{"vessel_id" => vessel_id.clone()}).map(|result| 
            result.map(|x| x.unwrap())).unwrap();

    rows.map(|row|  mysql::from_row::<usize>(row)).sum()
}

fn get_sum_of_animals_for_vessel(vessel_id: String, db_pool: &mysql::Pool) -> usize {
    let query = "SELECT Sum(o.count) FROM observation o INNER JOIN transect t ON o.transect_id=t.id WHERE t.vessel_id=:vessel_id AND o.obs_type='Sighting'";

    let rows = db_pool.prep_exec(query, params!{"vessel_id" => vessel_id.clone()}).map(|result| 
            result.map(|x| x.unwrap())).unwrap();

    rows.map(|row|  mysql::from_row::<Option<usize>>(row)).flatten().sum()
}

pub fn get_observations(transect_id: String, db_pool: &mysql::Pool) -> Vec<model::Observation> {
    let query = "SELECT id, obs_type, date, bearing, count, lat, lon, distance_km, group_type, \
        beaufort_type, weather_type from observation WHERE transect_id = :transect_id";

    let rows = db_pool.prep_exec(query, params!{"transect_id" => transect_id.clone()}).map(|result| 
            result.map(|x| x.unwrap())).unwrap();

    rows.map(|row| {
        let (id, obs_type, date, bearing, count, lat, lon, distance_km, 
            group_type, beaufort_type, weather_type) = 
            mysql::from_row::<(String, String, NaiveDateTime, Option<usize>, Option<usize>, f32, f32, Option<f32>, Option<String>, Option<String>, Option<String>)>(row);

        model::Observation{
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

pub fn get_transect_by_id(transect_id: String, db_pool: &mysql::Pool) -> Option<model::Transect> {
    let query = "SELECT id, start_date, end_date, bearing, start_lat, start_lon, end_lat, \
        end_lon, vessel_id, observer1_id, observer2_id from transect WHERE id = :id";

    let params = params! {"id" => &transect_id};
    let mut rows = db_pool.prep_exec(query, params).map(
        |result| result.map(|x| x.unwrap())).unwrap();

    rows.next().map(|row| {
        let (id, start_date, end_date, bearing, 
            start_lat, start_lon, end_lat, end_lon, vessel_id, 
            observer1_id, observer2_id) = 
            mysql::from_row::<(String, NaiveDateTime, NaiveDateTime, i32, f32, f32, f32, f32, String, String, Option<String>)>(row);

        let observations = get_observations(transect_id, &db_pool);

        model::Transect{
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
    })
}

pub fn upsert_transect(transect: &model::Transect, db_pool: &mysql::Pool) {
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

}

fn get_transects_for_vessel(vessel_id: String, db_pool: &mysql::Pool) -> Vec<model::Transect> {
    let query = "SELECT id, start_date, end_date, bearing, start_lat, \
        start_lon, end_lat, end_lon, observer1_id, observer2_id from transect WHERE vessel_id=:vessel_id";

    let params = params! {"vessel_id" => &vessel_id};
    let rows = db_pool.prep_exec(query, params).map(|result| {
            result.map(|x| x.unwrap())}).unwrap();
    rows.map(|row| {
        let (id, start_date, end_date, bearing, 
            start_lat, start_lon, end_lat, end_lon,  
            observer1_id, observer2_id) = 
            mysql::from_row::<(String, NaiveDateTime, NaiveDateTime, i32, f32, f32, f32, f32, String, Option<String>)>(row);

        model::Transect{
            id,
            start_date: start_date.timestamp(),
            end_date: end_date.timestamp(),
            start_lat,
            start_lon,
            end_lat,
            end_lon,
            vessel_id: vessel_id.clone(),
            bearing,
            observer1_id,
            observer2_id,
            observations: vec![]
        }
    }).collect()
}

pub fn get_all_transects(db_pool: &mysql::Pool) -> Vec<model::Transect> {
    let query = "SELECT id, start_date, end_date, bearing, start_lat, \
        start_lon, end_lat, end_lon, vessel_id, observer1_id, observer2_id from transect";
    let rows = db_pool.prep_exec(query, ()).map(|result| {
            result.map(|x| x.unwrap())}).unwrap();
    rows.map(|row| {
        let (id, start_date, end_date, bearing, 
            start_lat, start_lon, end_lat, end_lon, vessel_id, 
            observer1_id, observer2_id) = 
            mysql::from_row::<(String, NaiveDateTime, NaiveDateTime, i32, f32, f32, f32, f32, String, String, Option<String>)>(row);

        let observations = get_observations(id.clone(), &db_pool);

        model::Transect{
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
