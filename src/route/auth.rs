use crate::auth::get_session_from_token;
use crate::route::session::SESSION_COOKIE_NAME;
use log::{debug, error};
use tower_cookies::Cookies;

pub fn is_request_permitted(auth_enabled: bool, secret: &str, cookies: &Cookies) -> bool {
    let mut permit_request = false;

    debug!("authentication enabled: {auth_enabled}");

    if auth_enabled {

        if let Some(cookie) = cookies.get(SESSION_COOKIE_NAME) {
            let session_token = cookie.value();

            match get_session_from_token(&secret, &session_token) {
                Ok(_) => permit_request = true,
                Err(e) => {
                    error!("{}", e);
                }
            }

        } else {
            debug!("session cookie wasn't found");
        }

    } else {
        permit_request = true;
    }

    debug!("permit request: {permit_request}");

    permit_request
}