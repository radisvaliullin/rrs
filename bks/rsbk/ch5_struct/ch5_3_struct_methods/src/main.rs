#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self not mutable (only read)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // checks that self rectangle can hold entire other rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // assosiated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("struct methods");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("can rec1 hold rec2: {}", rect1.can_hold(&rect2));
    println!("can rec1 hold rec3: {}", rect1.can_hold(&rect3));

    // like constructor/new
    let sq1 = Rectangle::square(42);
    println!("square:\n{:#?}", sq1);
}
