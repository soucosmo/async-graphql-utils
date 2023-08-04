use serde::Deserialize;
use std::clone::Clone;


#[derive(Deserialize, Clone)]
pub struct KeycloakConfig {
    pub server_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub realm_name: String,
}
