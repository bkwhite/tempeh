use crate::Post;
use fake::{
    Fake,
    faker::{chrono::en::DateTime, internet::en::Username, lorem::en::Words},
};

pub fn random_post(id: String) -> Post {
    Post {
        id,
        username: Username().fake(),
        content: Words(5..10).fake::<Vec<String>>().join(" "),
        created: DateTime().fake(),
    }
}
