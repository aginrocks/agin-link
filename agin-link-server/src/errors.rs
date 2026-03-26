use axum::{extract::rejection::JsonRejection, response::IntoResponse};
use color_eyre::eyre::Report;
use sea_orm::DbErr;
use serde::Serialize;
use strum::{Display, EnumDiscriminants};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug, EnumDiscriminants)]
#[strum_discriminants(name(ErrorCode))]
#[strum_discriminants(derive(Serialize, Display, ToSchema))]
#[strum_discriminants(vis(pub))]
pub enum ApiError {
    #[error("Invalid body: {0}")]
    JsonRejection(#[from] JsonRejection),

    #[error("Database error: {0}")]
    Database(#[from] DbErr),

    #[error(transparent)]
    Unknown(#[from] Report),
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Machine-readable error code
    pub code: ErrorCode,

    /// Human-readable message
    pub message: String,

    /// Optional key-value details
    pub details: Option<serde_json::Value>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
