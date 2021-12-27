use hello_rust::format;

fn main() {
    println!("Hello, world!");

    format::print_format();
}

// copy and paste
// https://www.rust-lang.org/ja/learn/get-started

// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};

// fn main() {
    // let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans!");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());
    // say(message.as_bytes(), width, &mut writer).unwrap();
// }    
