use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation, TokenData};
use std::collections::HashMap;
use serde_json::{Value, json};

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

    pub fn get_token_claims(&self) -> Value {
        match self.get_access_token() {
            Some(token) => {
                let key = DecodingKey::from_secret(&[]);

                let mut validation = Validation::new(Algorithm::HS256);

                validation.insecure_disable_signature_validation();

                let data: TokenData<Value> = decode(
                    token.as_str(),
                    &key,
                    &validation
                ).unwrap();

                data.claims
            },
            None => json!({}),
        }
    }
}
