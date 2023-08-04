use super::{KeycloakConfig, AuthorizationBearer};
use async_graphql::{Guard, Context, Result};
use rs_keycloak::client::OpenID;


pub struct KeycloakGuard {
    check_roles: Vec<String>,
    check_all_roles: bool,
}

impl KeycloakGuard {
    pub fn new(check_roles: &[&str], check_all_roles: bool) -> KeycloakGuard {
        Self {
            check_roles: check_roles.iter().map(|i| i.to_string()).collect(),
            check_all_roles: check_all_roles,
        }
    }
}

#[async_trait::async_trait]
impl Guard for KeycloakGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let keycloak: &KeycloakConfig = ctx.data::<KeycloakConfig>().unwrap();

        let bearer: &AuthorizationBearer = ctx.data::<AuthorizationBearer>().unwrap();

        let bearer = bearer.get_access_token();

        if bearer.is_none() {
            return Err("Token bearer not informed".into());
        }

        let open_id = OpenID::introspect(
            keycloak.server_url.as_str(),
            keycloak.realm_name.as_str(),
            keycloak.client_id.as_str(),
            keycloak.client_secret.as_str(),
            bearer.unwrap().as_str(),
        );

        let check_roles = &self.check_roles.iter().map(|i| i.as_str()).collect::<Vec<&str>>();

        if self.check_all_roles {
            match open_id {
                Ok(openid) => if openid.has_all_roles(check_roles) {
                    Ok(())
                } else {
                    Err("Unauthorized".into())
                },
                Err(e) => Err(e.into()),
            }
        } else {
            match open_id {
                Ok(openid) => if openid.has_any_roles(check_roles) {
                    Ok(())
                } else {
                    Err("Unauthorized".into())
                },
                Err(e) => Err(e.into()),
            }
        }
    }
}
