mod links;

use utoipa::{IntoResponses, ToSchema};
use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let auth = OpenApiRouter::new().nest("/links", links::routes());

    // let public = OpenApiRouter::new()
    //     .nest("/discovery", discovery::routes())
    //     .nest("/login", login::routes());

    auth
}

/// Unauthorized
#[derive(ToSchema)]
#[schema(example = json!({ "error": "Unauthorized" }))]
pub struct UnauthorizedError {
    pub error: String,
}
