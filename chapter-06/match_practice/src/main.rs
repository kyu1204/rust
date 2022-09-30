#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    let a = Coin::Quarter(UsState::Alabama);
    println!("coin value is {}", value_in_cents(a));

    let b = Coin::Quarter(UsState::Alaska);
    let mut count = 0;

    if let Coin::Quarter(state) = b {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("count value is {}", count);
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {:?}", state);
            25
        }
    }
}
