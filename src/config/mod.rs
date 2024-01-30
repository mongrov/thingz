use config::{Config, ConfigError, Environment, File};
use log::info;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Logs {
    pub path: String,
    pub files: usize,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct MQTT {
    pub host: String,
    pub port: u16,
    pub topic: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub mqtt: MQTT,
    pub logs: Logs,
}

impl Settings {
    pub fn new(cfg_file: String) -> Result<Self, ConfigError> {
        let cfg_file_path = if cfg_file.is_empty() {
            "config/default"
            // let home = env::var("HOME").unwrap();
            // format!("{}{}", home, "/.thingz")
        } else {
            &cfg_file
        };
        info!("Config file as {:?}", cfg_file_path);
        let s = Config::builder()
            .add_source(File::with_name(cfg_file_path))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            // You may also programmatically change settings
            // .set_override("database.url", "postgres://")?
            .build()?;

        // Now that we're done, let's access our configuration
        // println!("debug: {:?}", s.get_bool("debug"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
