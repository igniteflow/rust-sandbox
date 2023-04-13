/*
   ❯ rustc enum.rs && ./enum
   🔴
   🟡
   🟢
*/
#![allow(dead_code)]
enum TrafficLight {
    Red,
    Amber,
    Green,
}

fn traffic_light_status(light: TrafficLight) {
    // Emoji decimals bytes from https://symbl.cc/en/
    let red = String::from_utf8(vec![240, 159, 148, 180]).unwrap();
    let amber = String::from_utf8(vec![240, 159, 159, 161]).unwrap();
    let green = String::from_utf8(vec![240, 159, 159, 162]).unwrap();

    match light {
        TrafficLight::Green => println!("{green}"),
        TrafficLight::Amber => println!("{amber}"),
        TrafficLight::Red => println!("{red}"),
    }
}

fn traffic_light_status_with_use(light: TrafficLight) {
    let red = String::from_utf8(vec![240, 159, 148, 180]).unwrap();
    let amber = String::from_utf8(vec![240, 159, 159, 161]).unwrap();
    let green = String::from_utf8(vec![240, 159, 159, 162]).unwrap();

    // The use declaration can be used to bind a full path to a new name, for easier access
    use crate::TrafficLight::*;
    match light {
        Green => println!("{green}"),
        Amber => println!("{amber}"),
        Red => println!("{red}"),
    }
}

fn main() {
    let green = TrafficLight::Green;
    let amber = TrafficLight::Amber;
    let red = TrafficLight::Red;

    traffic_light_status_with_use(red);
    traffic_light_status_with_use(amber);
    traffic_light_status_with_use(green);
}
