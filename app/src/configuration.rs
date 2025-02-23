use pavex::blueprint::Blueprint;
use pavex::t;

pub fn register(bp: &mut Blueprint) {
    bp.prebuilt(t!(self::AppConfig));
}

#[derive(serde::Deserialize, Debug, Clone)]
/// The configuration object holding all the values required
/// to configure the application.
pub struct AppConfig {}
