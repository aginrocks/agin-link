use axum::{Extension, Json};
use axum_valid::Valid;
use entity::link::Model as Link;
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use validator::Validate;

use crate::{errors::AxumResult, routes::api::UnauthorizedError, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(login))
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateLinkRequest {
    #[validate(length(min = 1, max = 2048))]
    pub oidc_token: String,

    #[validate(length(min = 1, max = 64))]
    pub device_name: String,
}

/// Create link
///
/// Creates a link that will be accessible at `{base_url}/{slug}`
#[utoipa::path(
    method(post),
    path = "/",
    responses(
        (status = OK, description = "Success", body = Link, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
    ),
    tag = "Links"
)]
async fn login(
    Extension(state): Extension<AppState>,
    Valid(Json(body)): Valid<Json<CreateLinkRequest>>,
) -> AxumResult<Json<Link>> {
    todo!()
}
