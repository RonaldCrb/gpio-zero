use actix_web::{
  HttpResponse,
  Responder
};

use super::super::relays;

pub async fn get_relays() -> impl Responder {
  let r = relays::new_relay(String::from("Relay 1"), 26);
  r.play();
  HttpResponse::Ok().body("<h1>get all relays status</h1>")
}

pub async fn get_relay_by_id() -> impl Responder {
  HttpResponse::Ok().body("<h1>Relay {id} status</h1>")
}

pub async fn turn_on_relay_by_id() -> impl Responder {
  // let rel = relays.new(26);
  HttpResponse::Ok().body("<h1>relay {id} on</h1>")
}

pub async fn turn_off_relay_by_id() -> impl Responder {
  HttpResponse::Ok().body("<h1>Relay #{id} off</h1>")
}
