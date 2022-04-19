use serde::Deserialize;

#[derive(Debug)]
pub enum AppCfgError {
    Other(String),
}

#[derive(Deserialize, Debug)]
pub struct AppCfg {
    pub dist: String,
    pub agents: Vec<String>,
}

pub fn load_app_cfg() -> Result<AppCfg, AppCfgError> {
    toml::from_str(include_str!("../cfg.toml")).map_err(|e| AppCfgError::Other(e.to_string()))
}
