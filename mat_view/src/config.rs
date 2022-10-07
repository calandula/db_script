use serde::Deserialize;

const DEFAULT_DB_PATH: &str = "mongodb://localhost:27017";
const DEFAULT_DB_NAME: &str = "prestalo";
const APP_NAME: &str = "Materialized view generator";
const VIEW_NAME: &str = "app_view";

#[derive(Deserialize, Debug)]

pub struct Configuration {
    #[serde(default="default_db_path")]
    pub db_path: String, 
    #[serde(default="default_db_name")]
    pub db_name: String,
    #[serde(default="default_app_name")]
    pub app_name: String,
    #[serde(default="default_view_name")]
    pub view_name: String
}

fn default_db_path() -> String {
    String::from(DEFAULT_DB_PATH)
}

fn default_db_name() -> String { 
    String::from(DEFAULT_DB_NAME)
} 

fn default_app_name() -> String {
    String::from(APP_NAME)
}

fn default_view_name() -> String {
    String::from(VIEW_NAME)
}