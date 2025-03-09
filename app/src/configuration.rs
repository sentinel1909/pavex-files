use pavex::blueprint::Blueprint;
use pavex::t;
use std::borrow::Cow;

pub fn register(bp: &mut Blueprint) {
    bp.prebuilt(t!(self::AppConfig));
}

#[derive(serde::Deserialize, Debug, Clone)]
/// The configuration object holding all the values required
/// to configure the application.
pub struct AppConfig {
   pub static_files: StaticFilesConfig, 
}

// methods for the AppConfig type
impl AppConfig {
    pub fn static_files_config(&self) -> &StaticFilesConfig {
        &self.static_files
    }
}

// struct type to represent the assets storage configuration
#[derive(serde::Deserialize, Clone, Debug)]
pub struct StaticFilesConfig {
    pub dir: Cow<'static, str>,
}
