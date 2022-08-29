struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color(u32, u32, u32);
struct Point(u32, u32, u32);

fn main() {
    let user = User {
        username: String::from("test"),
        email: String::from("test@test.com"),
        sign_in_count: 1,
        active: true
    };
   println!("user username value is {}", user.username);

    let mut user2 = User {
        username: "test".to_string(),
        email: "test@test.com".to_string(),
        sign_in_count: 1,
        active: true
    };
    user2.username = String::from("test2");
    println!("user2 username value is {}", user2.username);

    let user3 = build_user(String::from("test@test.com"), "test3".to_string());
    println!("user3 username value is {}", user3.username);

    let user4 = User {
        email: "test@test.com".to_string(),
        username: "test4".to_string(),
        ..user3
    };
    println!("user4 username value is {}", user4.username);

    let color = Color(1, 1, 1);
    let point = Point(2, 2, 2);
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true
    }
}