use crate::customers;
use crate::mongo_connection::Pool;
use customers::Customer;
use mongodb::oid::ObjectId;
use actix_web::{HttpResponse, Error, web};

pub async fn get_customers(conn: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let cs = customers::repository::all(&conn);
    Ok(HttpResponse::Ok().json(cs.unwrap()))
}

pub async fn get_customer_by_id(user_id: web::Path<String>, conn: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let c = customers::repository::get(ObjectId::with_string(&user_id).unwrap(), &conn);
    Ok(HttpResponse::Ok().json(c.unwrap()))
}

pub async fn add_customer(c: web::Json<Customer>, conn: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let c = customers::repository::insert(c.into_inner(), &conn);
    Ok(HttpResponse::Ok().json(c.unwrap()))
}

pub async fn update_customer(user_id: web::Path<String>, c: web::Json<Customer>, conn: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let c = customers::repository::update(ObjectId::with_string(&user_id).unwrap(), c.into_inner(), &conn);
    Ok(HttpResponse::Ok().json(c.unwrap()))
}

pub async fn delete_customer(user_id: web::Path<String>, conn: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let c = customers::repository::delete(ObjectId::with_string(&user_id).unwrap(), &conn);
    Ok(HttpResponse::Ok().json(c.unwrap().deleted_count))
}