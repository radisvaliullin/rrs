
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("structs");

    // define and init struct
    let mut usr = User {
        active: true,
        username: String::from("user73"),
        email: String::from("u73@user.com"),
        sign_in_count: 1,
    };
    println!("user - {:?}", usr);
    println!("user fields: username - {:?}; email - {}; sign_in_count - {}", usr.username, usr.email, usr.sign_in_count);

    // set field new value
    usr.active = false;
    println!("user not active - {:?}", usr);

    // init new object from other
    let usr2 = User {
        ..usr
    };
    println!("user 2 from user - {:?}", usr2);

    // or
    let usr3 = User {
        username: String::from("user33"),
        ..usr2
    };
    println!("user 3 from user - {:?}", usr3);

    // tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black - {:?}", black);
    println!("origin - {:?}", origin);

    // unit like struct
    let subject = AlwaysEqual;
    println!("subject - {:?}", subject);
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;
