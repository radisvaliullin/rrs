use ch17_3_oop_patern::{Post, PostV2};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    println!("post cont: {}", post.content());
    post.request_review();
    post.approve();
    println!("post cont: {}", post.content());

    // V2
    let mut post = PostV2::new();
    post.add_text("I ate a soup for lunch today");
    let post = post.request_review();
    let post = post.approve();
    println!("post v2 cont: {}", post.content());
}
