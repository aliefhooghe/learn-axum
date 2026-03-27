use utoipa::openapi::security::{AuthorizationCode, Flow, OAuth2, Scopes, SecurityScheme};
use utoipa::{Modify, OpenApi};

// Integrate authentication in swagger
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "oidc",
            SecurityScheme::OAuth2(OAuth2::new([Flow::AuthorizationCode(
                AuthorizationCode::new(
                    "http://localhost:8080/realms/master/protocol/openid-connect/auth",
                    "http://localhost:8080/realms/master/protocol/openid-connect/token",
                    Scopes::from_iter([
                        ("openid", "OpenID Connect"),
                        ("profile", "Profile"),
                        ("email", "Email"),
                    ]),
                ),
            )])),
        );
    }
}
#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
