#[derive(Debug)]
enum Message {
    Quit,
    // commented because get dead code worning
    // Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        println!("message: {:#?}", self)
    }
}

fn main() {
    println!("emuns");

    let mq = Message::Quit;
    mq.print();
    // let mm = Message::Move{x: 2, y: 84};
    // mm.print();
    let mw = Message::Write(String::from("qwerty"));
    mw.print();
    let mcc = Message::ChangeColor(1, 2, 3);
    mcc.print();

    // commented, return panic
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;
    // println!("sum = {}", sum);
}
