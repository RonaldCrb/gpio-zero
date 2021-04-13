use rust_gpiozero;
use std::{time, thread};

/// single relay representation
pub struct Relay {
  /// wether the relay is on or off
  pub is_on: bool,
  /// the pin number of the relay
  pub pin: u8,
  /// the name of the relay
  pub name: String,
  /// the actual relay
  pub relay: rust_gpiozero::OutputDevice,
  /// when was the last event registered timestamp in unix
  pub last_event: String
}

pub struct RelayInterface {
  pub name: &'static str,
  pub interfaces: Vec<Relay>, 
}

static mut RELAY_INTERFACE: RelayInterface = RelayInterface {
  name: "Main Relay Interface",
  interfaces: vec![]
};

pub fn get_relay_status() -> String {
  unsafe {
    format!(
      "Relay 1 => {}\nRelay 2 => {}\nRelay 3 => {}\n", 
      RELAY_INTERFACE.interfaces[0].is_on, 
      RELAY_INTERFACE.interfaces[1].is_on, 
      RELAY_INTERFACE.interfaces[2].is_on
    )
  }
}

pub fn init_relays() {
  let ch1 = new_relay(String::from("Relay 1"), 26);
  let ch2 = new_relay(String::from("Relay 2"), 20);
  let ch3 = new_relay(String::from("Relay 3"), 21);
  
  unsafe {
    RELAY_INTERFACE.interfaces = vec![ch1, ch2, ch3]
  }

}

pub fn new_relay(name: String, pin: u8) -> Relay {
  //! Instantiate a new Relay object 
  let relay = rust_gpiozero::OutputDevice::new(pin);
  let timestamp = time::SystemTime::now();
  Relay {
    is_on: false,
    pin,
    name,
    relay,
    last_event: format!("[CREATED] => {:?}", timestamp) 
  }
}

pub fn toggle_relay_by_index(index: usize) {
  //! Instantiate a new Relay object 
  unsafe {
    let mut r: &Relay = &RELAY_INTERFACE.interfaces[index];
    if r.is_on {
      r.is_on = false;
      r.relay.off();
    } else {
      r.is_on = true;
      r.relay.on();
    }
  }
}

impl Relay {

  pub fn play(&self) {
    // create new led attached to pin 17
    let mut relay = rust_gpiozero::OutputDevice::new(self.pin); // OutputDevice::new(self.pin);
    let t = time::Duration::from_secs(1);
    let mut count = 0;

    loop {
        count += 1;
        relay.on();
        thread::sleep(t);
        relay.off();
        thread::sleep(t);
        if count == 3 {
          break;
        }
    }
  }

}