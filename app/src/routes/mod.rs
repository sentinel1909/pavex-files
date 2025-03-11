pub mod ping;
pub mod static_files;

use pavex::blueprint::{Blueprint, router::GET};
use pavex::f;

// Blueprint for the file server
fn website_bp() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.route(GET, "/{filename}", f!(self::static_files::get));
    bp
}

// Blueprint for the API
fn api_bp() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.route(GET, "/ping", f!(self::ping::get));
    bp
}

// merge the Blueprints together under domain guards
pub fn register(bp: &mut Blueprint) {
    bp.domain("pavex-files.local").prefix("/public_html").nest(website_bp());

    bp.domain("api.pavex-files.local").prefix("/v1").nest(api_bp());
}
