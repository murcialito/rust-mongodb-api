#![allow(proc_macro_derive_resolution_fallback)]

pub mod handler;
pub mod repository;
use mongodb as bson;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub age: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableCustomer {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub age: Option<i32>,
}

impl InsertableCustomer {
    fn from_customer(customer: Customer) -> InsertableCustomer {
        InsertableCustomer {
            firstname: customer.firstname,
            lastname: customer.lastname,
            age: customer.age,
        }
    }
}
