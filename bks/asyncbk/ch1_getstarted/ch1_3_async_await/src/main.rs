use futures::executor::block_on;

async fn hello_world() {
    println!("async hi. Ыыы!")
}

fn main() {
    // none blocking just return future, none execute
    let future = hello_world();
    // future is run and execture
    block_on(future);

    block_on(async_all());
}

async fn async_all() {
    let ot = one_two();
    let th = three();

    futures::join!(ot, th);
}

async fn one_two() {
    let o = one().await;
    println!("one done: {}", o);
    two().await;
}

async fn one() -> isize {
    for i in 1..5 {
        println!("one i is {}", i);
    }
    1
}

async fn two() -> isize {
    for i in 1..5 {
        println!("two: i is {}",i);
    }
    2
}

async fn three() -> isize {
    for i in 1..5 {
        println!("three: i is {}",i);
    }
    3
}
