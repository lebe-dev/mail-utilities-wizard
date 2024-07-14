use crate::VERSION;

pub async fn get_version_route() -> &'static str {
    VERSION
}