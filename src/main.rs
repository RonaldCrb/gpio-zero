mod handlers;
mod relays;

use actix_web::{web, App, HttpServer, HttpResponse};


/// MAIN DOCS =====> https://www.waveshare.com/wiki/RPi_Relay_Board <=====
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();

  let port = std::env::var("PORT").unwrap_or("8080".to_string());
  let address = format!("127.0.0.1:{}", port);

  println!("Initializing Relays");
  relays::init_relays();
  

  println!("starting server on http://{}", address);
  HttpServer::new(|| {
    App::new()
      .route("/relays", web::get().to(handlers::relays::get_relays))
      .route("/relays/{id}", web::get().to(handlers::relays::get_relay_by_id))
      .route("/relays/{id}/on", web::get().to(handlers::relays::turn_on_relay_by_id))
      .route("/relays/{id}/off", web::get().to(handlers::relays::turn_off_relay_by_id))
      .route("/healthz", web::get().to(|| HttpResponse::Ok()))
  })
  .bind(&address)?
  .run()
  .await
}
