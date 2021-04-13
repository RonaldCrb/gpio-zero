use rust_gpiozero;
use std::{time, thread};

/// single relay representation
pub struct Relay {
  /// wether the relay is on or off
  is_on: bool,
  /// the pin number of the relay
  pin: u8,
  /// the name of the relay
  name: String,
  /// when was the last event registered timestamp in unix
  last_event: String
}

pub fn new_relay(name: String, pin: u8) -> Relay {
  //! Instantiate a new Relay object 
  let last_event = time::SystemTime::now();
  Relay {
    is_on: false,
    pin,
    name,
    last_event: format!("[CREATED] => {:?}", last_event) 
  }
}

impl Relay {

  pub fn play(&self) {
    // create new led attached to pin 17
    let mut relay = rust_gpiozero::OutputDevice::new(self.pin); // OutputDevice::new(self.pin);
    let t = time::Duration::from_millis(500);
    let mut count = 0;

    loop {
        count += 1;
        relay.on();
        thread::sleep(t);
        relay.off();
        thread::sleep(t);
        if count == 10 {
          break;
        }
    }
  }

}