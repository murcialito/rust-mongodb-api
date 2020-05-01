#![allow(proc_macro_derive_resolution_fallback)]
use crate::customers::{Customer, InsertableCustomer};
use crate::mongo_connection::Pool;
use r2d2_mongodb::mongodb::db::ThreadedDatabase;
use mongodb::{bson, coll::results::DeleteResult, doc, error::Error, oid::ObjectId}; 

const COLLECTION: &str = "customers";

pub fn all(connection: &Pool) -> Result<Vec<Customer>, Error> {
    let cursor = connection.get().unwrap().collection(COLLECTION).find(None, None).unwrap();

    cursor
        .map(|result| match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => Ok(result_model),
                Err(_) => Err(Error::DefaultError(String::from(""))),
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<Customer>, Error>>()
}

pub fn get(id: ObjectId, connection: &Pool) -> Result<Option<Customer>, Error> {
    match connection.get().unwrap()
        .collection(COLLECTION)
        .find_one(Some(doc! {"_id": id}), None)
    {
        Ok(db_result) => match db_result {
            Some(result_doc) => match bson::from_bson(bson::Bson::Document(result_doc)) {
                Ok(result_model) => Ok(Some(result_model)),
                Err(_) => Err(Error::DefaultError(String::from(
                    "Failed to create reverse BSON",
                ))),
            },
            None => Ok(None),
        },
        Err(err) => Err(err),
    }
}

pub fn insert(customers: Customer, connection: &Pool) -> Result<ObjectId, Error> {
    let insertable = InsertableCustomer::from_customer(customers.clone());
    match bson::to_bson(&insertable) {
        Ok(model_bson) => match model_bson {
            bson::Bson::Document(model_doc) => {
                match connection.get().unwrap()
                    .collection(COLLECTION)
                    .insert_one(model_doc, None)
                {
                    Ok(res) => match res.inserted_id {
                        Some(res) => match bson::from_bson(res) {
                            Ok(res) => Ok(res),
                            Err(_) => Err(Error::DefaultError(String::from("Failed to read BSON")))
                        },
                        None => Err(Error::DefaultError(String::from("None")))
                    },
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::DefaultError(String::from(
                "Failed to create Document",
            ))),
        },
        Err(_) => Err(Error::DefaultError(String::from("Failed to create BSON"))),
    }
}

pub fn update(id: ObjectId, customers: Customer, connection: &Pool) -> Result<Customer, Error> {
    let mut new_customer = customers.clone();
    new_customer.id = Some(id.clone());
    match bson::to_bson(&new_customer) {
        Ok(model_bson) => match model_bson {
            bson::Bson::Document(model_doc) => {
                match connection.get().unwrap()
                    .collection(COLLECTION)
                    .replace_one(doc! {"_id": id}, model_doc, None)
                {
                    Ok(_) => Ok(new_customer),
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::DefaultError(String::from(
                "Failed to create Document",
            ))),
        },
        Err(_) => Err(Error::DefaultError(String::from("Failed to create BSON"))),
    }
}

pub fn delete(id: ObjectId, connection: &Pool) -> Result<DeleteResult, Error> {
    connection.get().unwrap()
        .collection(COLLECTION)
        .delete_one(doc! {"_id": id}, None)
}