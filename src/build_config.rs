use config::Config;

#[derive(Debug, Deserialize)]
pub struct Settings {
  pub port: i16,
  pub bind: String,
  pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub user: String,
    pub password: String,
    pub bind: String,
    pub port: i16,
    pub name: String,
}

pub fn create_config() -> Settings {
  Config::builder()
    .add_source(config::File::with_name("config/config"))
    .build()
    .unwrap()
    .try_deserialize::<Settings>()
    .unwrap()
}
