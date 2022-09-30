// #[derive(PartialEq, Debug)]
// // Declare Car struct to describe vehicle with four named fields
// struct Car {
//     color: String,
//     motor: Transmission,
//     roof: bool,
//     age: (Age, u32)
// }
//
// #[derive(PartialEq, Debug)]
// enum Age {New, Used}
//
// #[derive(PartialEq, Debug)]
// // Declare enum for Car transmission type
// enum Transmission { Manual, SemiAuto, Automatic }
//
//
// // Get the car quality by testing the value of the input argument
// // - miles (u32)
// // Create a tuple for the car quality with the Age ("New" or "Used") and mileage
// // Return a tuple with the arrow `->` syntax
// fn car_quality (miles: u32) -> (Age, u32) {
//     (Age::New, miles)
// }
//
// // Build a new "Car" using the values of four input arguments
// // - color (String)
// // - motor (Transmission enum)
// // - roof (boolean, true if the car has a hard top roof)
// // - miles (u32)
// // Call the car_quality(miles) function to get the car age
// // Return an instance of a "Car" struct with the arrow `->` syntax
// fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
//     // Create a new "Car" instance as requested
//     // - Bind first three fields to values of input arguments
//     // - "age" field calls "car_quality" function with "miles" input argument
//     Car {
//         color,
//         motor,
//         roof,
//         age: car_quality(miles),
//     }
// }
//
// fn main() {
//     let colors = ["Blue", "Green", "Red", "Silver"];
//
//     // Declare the car type and initial values
//     let mut car: Car;
//     let mut engine: Transmission = Transmission::Manual;
//
//     // Order 3 cars, one car for each type of transmission
//
//     // Car order #1: New, Manual, Hard top
//     car = car_factory(String::from(colors[0]), engine, true, 0);
//     println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
//
//     // Car order #2: Used, Semi-automatic, Convertible
//     engine = Transmission::SemiAuto;
//     car = car_factory(String::from(colors[1]), engine, false, 100);
//     println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
//
//     // Car order #3: Used, Automatic, Hard top
//     engine = Transmission::Automatic;
//     car = car_factory(String::from(colors[2]), engine, true, 200);
//     println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
// }
#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car { color: String, motor: Transmission, roof: bool, age: (Age, u32) }

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
// Declare enum for Car age
enum Age { New, Used }

//////////////////////////////////////////////////

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
fn car_quality (miles: u32) -> (Age, u32) {
    if miles == 0 {
        (Age::New, miles)
    }
    else {
        (Age::Used, miles)
    }
}

//////////////////////////////////////////////////

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {

    // Show details about car order
    // - Check if order is for Used or New car, then check the roof type
    // - Print details for New or Used car based on roof type
    if miles == 0 {
        if roof {
            println!("Build a new car: {:?}, {}, Hard top, {} miles\n", motor, color, miles);
        }
        else {
            println!("Build a new car: {:?}, {}, Convertible, {} miles\n", motor, color, miles);
        }
    }
    else {
        if roof {
            println!("Prepare a used car: {:?}, {}, Hard top, {} miles\n", motor, color, miles);
        }
        else {
            println!("Prepare a used car: {:?}, {}, Convertible, {} miles\n", motor, color, miles);
        }

    }

    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - Bind "age" to tuple returned from car_quality(miles)
    Car {
        color,
        motor,
        roof,
        age: car_quality(miles)
    }
}

fn main() {
    // Car order #1: New, Manual, Hard top
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);

    // Car order #2: Used, Semi-automatic, Convertible
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    // Car order #3: Used, Automatic, Hard top
    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}