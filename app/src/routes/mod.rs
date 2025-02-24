pub mod ping;
pub mod static_files;

use pavex::blueprint::{router::GET, Blueprint};
use pavex::f;

pub fn register(bp: &mut Blueprint) {
    bp.route(GET, "/public_html/{*filename}", f!(self::static_files::get));
    bp.route(GET, "/api/ping", f!(self::ping::get));
}
