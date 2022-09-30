use serde::{Serialize, Deserialize};

pub mod schema {
    #[derive(Serialize, Deserialize)]
    pub struct BlogPost {
        id: i32,
        title: String,
        body: String,
        published: bool,
    }
}
