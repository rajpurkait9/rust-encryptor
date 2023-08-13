use std::io;

mod encrypt;

use encrypt::Encryptable;
fn main() {
    let mut user_input = String::new();
    println!("enter the string which you want to encrypt : ");
    io::stdin()
        .read_line(&mut user_input)
        .expect("cannot read the input");
    println!("the user input is {}", user_input);
    let user_input = user_input.trim();
    let user_input = encrypt::rot13::Rot13 {
        data: user_input.to_string(),
    };
    let s = encrypt::rot13::Rot13::encrypt(&user_input);
    println!("{}", s);
}
