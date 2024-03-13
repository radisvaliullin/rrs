use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::fs;

fn main() {
    println!("recoverable errors");

    let greeting_file_result = File::open("qwerty.txt");
    let some = match greeting_file_result {
        Ok(_) => {
            println!("greeting file opened OK");
            "OK"
        }
        Err(error) => {
            println!("greeting file open error {:?}", error);
            "ERROR"
        }
    };
    println!("file open status: {}", some);

    //
    let greeting_file_result = File::open("qwerty.txt");
    let some = match greeting_file_result {
        Ok(_) => {
            println!("greeting file opened OK");
            "OK"
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("greeting file open error {:?}", "NOT_FOUND");
                "ERROR"
            }
            _ => {
                println!("greeting file open error {:?}", "OTHER");
                "ERROR"
            }
        },
    };
    println!("file open status: {}", some);

    // commented, raise panic with expect message
    // let _ = File::open("qwerty.txt").expect("qwerty.txt do not exist");

    //
    let res = read_username_from_file();
    println!("res - {:?}", res);

    //
    let res = read_username_from_file_v2();
    println!("res - {:?}", res);

    //
    let res = read_username_from_file_v3();
    println!("res - {:?}", res);

    //
    let res = read_username_from_file_v4();
    println!("res - {:?}", res);

    let last = last_char_of_first_line("qwerty asdf zcxv\nqaz wsx edc");
    println!("last - {:?}", last);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("qwerty.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username_file = File::open("qwerty.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_v4() -> Result<String, io::Error> {
    fs::read_to_string("qwerty.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
