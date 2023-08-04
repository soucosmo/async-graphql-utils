pub struct AuthorizationBearer {
    access_token: Option<String>,
}

impl AuthorizationBearer {
    pub fn get_access_token(&self) -> Option<String> {
        match &self.access_token {
            Some(token) => Some(token.replace("Bearer ", "")),
            None => None,
        }
    }
}
