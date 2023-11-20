use crate::models::Pizza;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Error, Surreal,
};

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("0.0.0.0:8000").await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;

        client.use_ns("surreal").use_db("pizzas").await.unwrap();

        Ok(Self {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("pizzas"),
        })
    }

    pub async fn get_all_pizzas(&self) -> Option<Vec<Pizza>> {
        let result = self.client.select("pizza").await;

        match result {
            Ok(all_pizza) => Some(all_pizza),
            Err(_) => None,
        }
    }

    pub async fn get_pizza_by_id(&self, pizza_id: String) -> Option<Pizza> {
        let pizza_to_show = self.client.select(("pizza", pizza_id)).await;

        match pizza_to_show {
            Ok(pizza) => pizza,
            Err(_) => None,
        }
    }

    pub async fn add_pizza(&self, new_pizza: Pizza) -> Option<Pizza> {
        let added_pizza = self
            .client
            .create(("pizza", new_pizza.uuid.clone()))
            .content(new_pizza)
            .await;

        match added_pizza {
            Ok(added) => added,
            Err(_) => None,
        }
    }

    pub async fn update_pizza_order(&self, uuid: String, new_amount: u32) -> Option<Pizza> {
        let find_pizza: Result<Option<Pizza>, Error> = self.client.select(("pizza", &uuid)).await;

        match find_pizza {
            Ok(found_pizza) => match found_pizza {
                Some(pizza) => {
                    let pizza_name = pizza.pizza_name;

                    let updated_pizza: Result<Option<Pizza>, Error> = self
                        .client
                        .update(("pizza", &uuid))
                        .merge(Pizza {
                            uuid,
                            pizza_name,
                            amount: new_amount,
                        })
                        .await;

                    match updated_pizza {
                        Ok(updated) => updated,
                        Err(_) => None,
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }

    pub async fn delete_pizza(&self, id_to_delete: String) -> Option<Pizza> {
        let pizza_to_delete = self
            .client
            .delete::<Option<Pizza>>(("pizza", id_to_delete))
            .await;

        match pizza_to_delete {
            Ok(result) => result,
            Err(_) => None,
        }
    }
}
