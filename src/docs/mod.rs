use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

/// Master OpenAPI structure aggregating all handlers, schemas, and security rules.
#[derive(OpenApi)]
#[openapi(
    paths(),
    components(
        schemas()
    ),
    modifiers(&SecurityAddon),
    tags()
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("Input your raw JSON Web Token (JWT)".to_string()))
                        .build(),
                ),
            );
        }
    }
}
