use crate::blog::model::blog_posts;
use crate::{BlogPost, DbCore};
use diesel::{QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;

#[post("/", data = "<blog_post>")]
pub async fn create_blog_post(connection: DbCore, blog_post: Json<BlogPost>) -> Json<BlogPost> {
    connection
        .run(move |c| {
            diesel::insert_into(blog_posts::table)
                .values(&blog_post.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("boo")
}

#[get("/")]
pub async fn get_all_blog_posts(connection: DbCore) -> Json<Vec<BlogPost>> {
    connection
        .run(|c| blog_posts::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog posts")
}

#[get("/<post_id>")]
pub async fn get_blog_post(connection: DbCore, post_id: i32) -> Json<BlogPost> {
    connection
        .run(move |c| blog_posts::table.find(post_id).get_result(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog post")
}
