use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message = "pizza name required"))]
    pub pizza_name: String,
    #[validate(range(min = 1))]
    pub amount: u32,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdatePizza {
    #[validate(range(min = 1))]
    pub new_amount: u32,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct DeletePizza {
    #[validate(length(min = 1, message = "uuid needed"))]
    pub uuid: String,
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Pizza {
    pub uuid: String,
    pub pizza_name: String,
    pub amount: u32,
}

impl Pizza {
    pub fn new(uuid: String, pizza_name: String, amount: u32) -> Self {
        Self {
            uuid,
            pizza_name,
            amount,
        }
    }
}
