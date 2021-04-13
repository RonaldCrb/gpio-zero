use actix_web::{
  HttpResponse,
  Responder
};


/*
Channel	| RPi Pin No | wiringPi | BCM | Descriptions
CH1	    |   37	     |    P25	  |  26 | Channel 1
CH2	    |   38	     |    P28	  |  20 | Channel 2
CH3	    |   40	     |    P29	  |  21 | Channel 3
*/

use super::super::relays;

pub async fn get_relays() -> impl Responder {
  let status = relays::get_relay_status();
  HttpResponse::Ok().body(status)
}

pub async fn get_relay_by_id() -> impl Responder {
  HttpResponse::Ok().body("<h1>Relay {id} status</h1>")
}

pub async fn turn_on_relay_by_id() -> impl Responder {
  HttpResponse::Ok().body("<h1>relay {id} on</h1>")
}

pub async fn turn_off_relay_by_id() -> impl Responder {
  HttpResponse::Ok().body("<h1>Relay #{id} off</h1>")
}
