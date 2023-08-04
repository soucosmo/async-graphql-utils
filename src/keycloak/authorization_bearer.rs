use std::collections::HashMap;

pub struct AuthorizationBearer {
    pub access_token: Option<String>,
    pub custom_data: Option<HashMap<String, String>>,
}

impl AuthorizationBearer {
    pub fn get_access_token(&self) -> Option<String> {
        match &self.access_token {
            Some(token) => Some(token.replace("Bearer ", "")),
            None => None,
        }
    }
}
