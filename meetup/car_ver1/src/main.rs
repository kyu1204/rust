// #[derive(Debug)]
// struct KeyPress(String, char);
//
// #[derive(Debug)]
// struct MouseClick {
//     x: i64,
//     y: i64
// }
//
// #[derive(Debug)]
// enum WebEvent {
//     WELoad(bool),
//     WEClick(MouseClick),
//     WEKeys(KeyPress)
// }
//
// fn main() {
//     let click = MouseClick {
//         x: 100,
//         y: 250
//     };
//     println!("Mouse Click location: {}, {}\n", click.x, click.y);
//
//     let keys = KeyPress(String::from("Ctrl+"), 'N');
//     println!("Keys pressed: {}{}\n",keys.0, keys.1);
//
//     let we_load = WebEvent::WELoad(true);
//     let we_click = WebEvent::WEClick(click);
//     let we_keys = WebEvent::WEKeys(keys);
//
//     println!("WebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_keys);
// }

// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car{
    Car {
        color,
        transmission,
        convertible,
        mileage: 0
    }
}

fn main() {
    // We have orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all the cars
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
}