use axum::{http::StatusCode, Json};

use serde::Serialize;
use serde_json::json;

#[derive(Debug)]
pub enum PizzaError {
    NoPizzasFound,
    PizzaCreationFailure,
    NoSuchPizzaFound,
    QuerryError,
}

#[derive(Serialize)]
struct ErrorResponse {
    status: &'static str,
    message: &'static str,
}

impl From<PizzaError> for (StatusCode, Json<serde_json::Value>) {
    fn from(value: PizzaError) -> Self {
        let (status, error_response) = match value {
            PizzaError::NoPizzasFound => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    status: "......",
                    message: "No pizza orders!!!",
                },
            ),
            PizzaError::PizzaCreationFailure => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error",
                    message: "There's an error with the server",
                },
            ),
            PizzaError::NoSuchPizzaFound => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    status: "not found",
                    message: "That kind of pizza is not found",
                },
            ),
            PizzaError::QuerryError => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    status: "not found",
                    message: "No matching ID in the database",
                },
            ),
        };

        (status, Json(json!(error_response)))
    }
}
