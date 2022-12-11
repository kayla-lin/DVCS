use std::io::{Stdin};
use std::panic::catch_unwind;
use std::{fs, io};
use std::fs::File;
use usemods::dir_c::create_dir_main;
use usemods::{ *};

fn main() {
   /*  println!("Hello, world!");
    const DVCS_HIDDEN: &str = "/tmp/dvcs_team";
   
    fs::create_dir(DVCS_HIDDEN);
    fs::create_dir("/tmp/dvcs_testi/");
    let _sfile = File::create("/tmp/dvcs_testi/"); */

    let cdr = create_dir_main();

    if cdr.is_err() {
        panic!("Error creating directory");
    }

    //taking user input 
    println!("Welcome to the DVCS:\n ");
    let mut input = String::new();

    let stdin: Stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    let mut input = input.split_whitespace();
    let command = String::from(input.next().unwrap());

    match command.as_str() {
        "init" => {
            let file_path = String::from(input.next().unwrap());
            user_interaction::init_in(file_path);

        },
        "diff" => {
            let file_path = String::from(input.next().unwrap());
            let head: String = String::from(input.next().unwrap());
            user_interaction::diff_in(file_path, head);
        },
        "status" => {
            let file_path = String::from(input.next().unwrap());
            user_interaction::status_in(file_path);
        },
        "add" => {
            let file_path = String::from(input.next().unwrap());
            user_interaction::add_in(file_path);
        },
        "remove" => {
            let file_path = String::from(input.next().unwrap());
            user_interaction::remove_in(file_path);
        },
        _ => {
            println!("Invalid command");
        }
    } 

}
