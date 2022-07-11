
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
    pub bearing: Option<i64>,
    pub count: Option<i64>,
    pub distance_km: Option<f32>,
    pub group_type: Option<String>,
    pub beaufort_type: Option<String>,
    pub weather_type: Option<String>,
}
