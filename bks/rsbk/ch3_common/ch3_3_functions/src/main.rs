fn main() {
    println!("main func");

    another_function(42);
    print_labeled_measurement(73, 'h');

    // expression as return value
    println!("five: {}", five());

    let i = {
        let x = 3;
        x + 1
    };
    println!("i is {i}");
}

fn another_function(x: i32) {
    println!("another function.");
    println!("the value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
