use std::sync::{Arc, Mutex};

use actix_web::{delete, get, post, web::{Data, Json}, HttpResponse, Responder};

use crate::{orderbook::Orderbook, inputs::{CreateOrderInput, DeleteOrder}};


#[get("/depth")]
pub async fn get_depth(orderbook: Data<Arc<Mutex<Orderbook>>>) -> impl Responder {
    let orderbook = orderbook.lock().unwrap();
    let depth = orderbook.get_depth();
    HttpResponse::Ok().json(depth)
}

#[post("/order")]
pub async fn create_order(orderbook: Data<Arc<Mutex<Orderbook>>>, order: Json<CreateOrderInput>) -> impl Responder {
    let mut orderbook = orderbook.lock().unwrap();
    let orderbook = orderbook.create_order(order.0);
    HttpResponse::Ok().json(orderbook)
}

#[delete("/order")]
pub async fn delete_order(orderbook: Data<Arc<Mutex<Orderbook>>>, order: Json<DeleteOrder>) -> impl Responder {
    let mut orderbook = orderbook.lock().unwrap();
    let orderbook = orderbook.delete_order(order.0);
    HttpResponse::Ok().json(orderbook)
}