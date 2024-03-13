use ch17_3_oop_patern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    println!("post cont: {}", post.content());
    post.request_review();
    post.approve();
    println!("post cont: {}", post.content());
}
