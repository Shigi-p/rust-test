// NOTE: that is test traits

// use hello_rust::format;
// use hello_rust::debug;
// use hello_rust::derive;
// use hello_rust::display;
// use hello_rust::formatting;
// use hello_rust::tuples;
use hello_rust::arrays_and_slices;

fn main() {
    // println!("Hello, world!");

    // NOTE: that is test traits

    // ---------- format
    // format::print_format();
    // format::print_format_sample();
    // format::print_debug();

    // ---------- debug
    // debug::test_debug();
    // debug::test_debug_object();

    // ---------- derive
    // derive::test_derive();

    // ---------- display
    // display::test_display();
    // display::test_display_complex_number();

    // ---------- formatting
    // formatting::formatting();

    // ---------- tuples
    // tuples::tuples_test();

    // ---------- arrays and slices
    arrays_and_slices::arrays_and_slices();
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
