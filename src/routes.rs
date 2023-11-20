use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use crate::db::Database;
use crate::errors::PizzaError;
use crate::models::{BuyPizzaRequest, DeletePizza, Pizza, UpdatePizza};

use uuid::Uuid;
use validator::Validate;

// ---- hold the database as a state

pub struct AppState {
    pub db: Database,
}

// ---- error type for the handler functions return type

type HandlerError = (StatusCode, Json<serde_json::Value>);

// ---- get (http://localhost:3000/get)

pub async fn get_orders(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Pizza>>, HandlerError> {
    let pizzas = state.db.get_all_pizzas().await;

    match pizzas {
        Some(found_pizzas) => Ok(Json(found_pizzas)),
        None => Err(PizzaError::NoPizzasFound.into()),
    }
}

// ---- get specific pizza (http://localhost:3000/get/uuid)

pub async fn get_pizza(
    State(state): State<Arc<AppState>>,
    Path(pizza_id): Path<String>,
) -> Result<Json<Pizza>, HandlerError> {
    let pizza = state.db.get_pizza_by_id(pizza_id).await;

    match pizza {
        Some(existed_pizza) => Ok(Json(existed_pizza)),
        None => Err(PizzaError::NoSuchPizzaFound.into()),
    }
}

// ---- post (http://localhost:3000/post)

pub async fn order_pizza(
    State(state): State<Arc<AppState>>,
    Json(body): Json<BuyPizzaRequest>,
) -> Result<(), HandlerError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name;
            let amount = body.amount;

            let mut buffer = Uuid::encode_buffer();
            let new_uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_pizza = state
                .db
                .add_pizza(Pizza::new(String::from(new_uuid), pizza_name, amount))
                .await;

            match new_pizza {
                Some(_) => Ok(()),
                None => Err(PizzaError::PizzaCreationFailure.into()),
            }
        }
        Err(_) => Err(PizzaError::PizzaCreationFailure.into()),
    }
}

// ---- patch (http://localhost:3000/update/:uuid)

pub async fn update_order(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<String>,
    Json(body): Json<UpdatePizza>,
) -> Result<(), HandlerError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let update_pizza = state.db.update_pizza_order(uuid, body.new_amount).await;

            match update_pizza {
                Some(_) => Ok(()),
                None => Err(PizzaError::PizzaCreationFailure.into()),
            }
        }
        Err(_) => Err(PizzaError::NoSuchPizzaFound.into()),
    }
}

// ---- delete (http://localhost:3000/delete)

pub async fn delete_pizza(
    State(state): State<Arc<AppState>>,
    Json(body): Json<DeletePizza>,
) -> Result<String, HandlerError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let id_to_delete = body.uuid;
            let deleted_pizza = state.db.delete_pizza(id_to_delete).await;

            match deleted_pizza {
                Some(pizza) => Ok(format!("successfuly deleted {:?}", pizza)),
                None => Err(PizzaError::QuerryError.into()),
            }
        }
        Err(_) => Err(PizzaError::QuerryError.into()),
    }
}
