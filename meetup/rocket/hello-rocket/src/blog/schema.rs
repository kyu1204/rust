use super::model::blog_posts;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "blog_posts"]
pub struct BlogPost {
    id: i32,
    title: String,
    body: String,
    published: bool,
}
impl BlogPost {
    pub fn new(id: i32, title: String, body: String, published: bool) -> Self {
        BlogPost {
            id,
            title,
            body,
            published,
        }
    }
}
