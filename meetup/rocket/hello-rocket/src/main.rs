mod blog;
mod lib;

use blog::api::{create_blog_post, get_all_blog_posts, get_blog_post};
use blog::schema::BlogPost;
use lib::db::DbCore;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

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
            routes![get_blog_post, create_blog_post, get_all_blog_posts],
        )
}
