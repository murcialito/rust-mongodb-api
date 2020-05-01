use actix_web::{web, App, HttpServer};

mod customers;
mod mongo_connection;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let pool = mongo_connection::init_pool();
    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/customers", web::get().to(customers::handler::get_customers))
            .route("/customers/{id}", web::get().to(customers::handler::get_customer_by_id))
            .route("/customers", web::post().to(customers::handler::add_customer))
            .route("/customers/{id}", web::put().to(customers::handler::update_customer))
            .route("/customers/{id}", web::delete().to(customers::handler::delete_customer))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}