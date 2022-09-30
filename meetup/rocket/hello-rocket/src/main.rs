use rocket::serde::json::Json;
use blog::schema::BlogPost;


#[macro_use] extern crate rocket;

#[get("/random")]
fn get_random_blog_post() -> Json<BlogPost> {
    Json(
        BlogPost {
            id: 1,
            title: "My first post".to_string(),
            body: "This is my first post".to_string(),
            published: true,
        }
    )
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    let rocket= rocket::build();

    rocket
        .mount("/", routes![index])
        .mount("/blog-posts", routes![get_random_blog_post])
}
