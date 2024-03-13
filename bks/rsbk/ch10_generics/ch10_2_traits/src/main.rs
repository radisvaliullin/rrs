use ch10_2_traits::{Comment, Pair, Some, Summary, Tweet};

fn main() {
    println!("treats");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    //
    let comm = Comment {
        author: String::from("qwerty"),
        message: String::from("asdf zxcv"),
    };
    println!("new comment: {}", comm.summarize());

    //
    notify(&tweet);
    notify2(&tweet);

    //
    println!("some func out: {}", some_function(&tweet, &comm));

    //
    println!(
        "return summarizable and get summarize: {}",
        returns_summarizable().summarize()
    );

    // pair
    let p = Pair::new(2, 3);
    println!("pair:");
    p.cmp_display();
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T, U>(t: &T, u: &U) -> usize
where
    T: Summary + Some,
    U: Summary + Some,
{
    t.summarize().len() + u.summarize().len() + t.some() + u.some()
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
