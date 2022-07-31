use crate::api::rest::{Comment, FullComment};

pub fn list(comments: &[Comment]) {
    println!("Comments:");
    for comment in comments {
        println!("-----");
        println!("{}", FullComment(comment));
    }
}
