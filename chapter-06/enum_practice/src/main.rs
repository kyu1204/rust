fn main() {
    // enum IpAddr {
    //     V4(String),
    //     V6(String)
    // }
    //
    // let home = IpAddr::V4("127.0.0.1".to_string());
    // let loopback = IpAddr::V6("::1".to_string());

    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String)
    // }
    //
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    #[derive(Debug)]
    enum Message {
        Quit,
        Move {x:i32, y:i32},
        Write(String),
        ChangeColor(i32, i32, i32)
    }
    impl Message {
        fn call(&self) {
            println!("{:?}", self)
        }
    }

    let m = Message::Write(String::from("Hello"));
    m.call();

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let z = Some("c");
    let xx: Option<String> = None;
}