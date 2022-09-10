
#[derive(Debug, Deserialize, Serialize)]
pub struct Transect {
    pub id: String,
    pub start_date: i64,
    pub end_date: i64,
    pub start_lat: f32,
    pub start_lon: f32,
    pub end_lat: f32,
    pub end_lon: f32,
    pub vessel_id: String,
    pub bearing: i32,
    pub observer1_id: String,
    pub observer2_id: Option<String>,
    pub observations: Vec<Observation>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Observation {
    pub id: String,
    pub transect_id: String,
    pub obs_type: String,
    pub date: i64,
    pub lat: f32,
    pub lon: f32,
    pub bearing: Option<usize>,
    pub count: Option<usize>,
    pub distance_km: Option<f32>,
    pub group_type: Option<String>,
    pub beaufort_type: Option<String>,
    pub weather_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Observer {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vessel {
    pub id: String,
    pub name: String,
    pub number_of_transects: usize,
    pub number_of_sightings: usize,
    pub total_duration_of_all_transects_secs: i64,
    pub total_distance_of_all_transects_km: f64,
    pub number_of_animals: usize,
}
