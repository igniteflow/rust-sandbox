/*
❯ rustc enum.rs && ./enum
🔴
🟡
🟢
 */

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

fn main() {
    let green = TrafficLight::Green;
    let amber = TrafficLight::Amber;
    let red = TrafficLight::Red;

    traffic_light_status(red);
    traffic_light_status(amber);
    traffic_light_status(green);
}
