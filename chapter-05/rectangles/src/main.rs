// Version1
// fn main() {
//     let width: u32 = 50;
//     let height: u32 = 30;
//
//     println!("rectangle area is {}", area(width, height));
// }
//
// fn area(width: u32, height: u32) -> u32{
//     width * height
// }

// Version2
// fn main() {
//     let rect: (u32, u32) = (50, 30);
//
//     println!("rectangle area is {}", area(rect));
// }
//
// fn area(dimension: (u32, u32)) -> u32{
//     dimension.0 * dimension.1
// }

// Version3
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32{
       self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle{width: size, height:size}
    }
}

fn main() {
   let rect = Rectangle {
       width: 50,
       height: 30
   };
    println!("rectangle area is {}", rect.area());
    println!("rect is {:?}", rect);

    let rect2 = Rectangle {
        width: 20,
        height: 20
    };
    let rect3 = Rectangle {
        width: 50,
        height: 50
    };
    println!("rect -> can_hold rect2 value is {}", rect.can_hold(&rect2));
    println!("rect -> can_hold rect3 value is {}", rect.can_hold(&rect3));

    println!("size 3 square value is {:#?}", Rectangle::square(3));
}
