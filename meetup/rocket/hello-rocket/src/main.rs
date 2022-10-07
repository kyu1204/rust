mod blog;
mod lib;

use blog::api::{create_blog_post, get_all_blog_posts};
use blog::schema::BlogPost;
use lib::db::DbCore;
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

#[get("/random")]
fn get_random_blog_post() -> Json<BlogPost> {
    Json(BlogPost::new(
        0,
        String::from("test"),
        String::from("body"),
        true,
    ))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();

    rocket
        .attach(DbCore::fairing())
        .mount("/", routes![index])
        .mount(
            "/blog-posts",
            routes![get_random_blog_post, create_blog_post, get_all_blog_posts],
        )
}
