use crate::api::rest::{Comment, FullComment};

pub fn list(comments: &[Comment]) {
    for comment in comments {
        println!("-----");
        println!("{}", FullComment(comment));
    }
}
