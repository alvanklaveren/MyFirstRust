
#![allow(unused_imports)]
mod first_tries;
use first_tries::tutorial::*;
mod web_server;
use web_server::single_threaded::*;

fn main() {
    println!("Hello, world!");

    //min_max();
    //guessing_game();
    //arrays();
    //tuples();
    //borrowing_test();
    //string_slice();
    //structs();
    //enums();
    //if_let();
    //strings_and_hash_maps();
    //error_handling();

    //let _res = match read_username_from_file() {
    //    Ok(fileName) => Ok(println!("okay then...")),
    //    Err(_) => Err(println!("well that failed miserably")),
    //};


    // webserver stuff
    listen();


    //count_words("some words in this sentence are repeated more than once, like some, this, and once.");
}
