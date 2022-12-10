use staging::*;
use std::io::{Stdin, BufRead};
use std::{fs, io};
use std::fs::File;
use usemods::{user_interaction::init_in, *};

fn main() {
    println!("Hello, world!");
    const DVCS_HIDDEN: &str = "/tmp/dvcs_team";
    fs::create_dir(DVCS_HIDDEN);
    fs::create_dir("/tmp/dvcs_testi/");
    let file = File::create("/tmp/dvcs_testi/");
    //taking user input 
    println!("Welcome to the DVCS:\n ");
    let mut input = String::new();

    let stdin: Stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    let mut input = input.split_whitespace();
    let command = String::from(input.next().unwrap());
// /tmp/dvcs_test
    match command.as_str() {
        "innit" => {
            let file_path = String::from(input.next().unwrap());
            user_interaction::init_in(file_path);

        },
        "add" => {
            let file_path = String::from(input.next().unwrap());
            user_interaction::add_in(file_path);
        },
        "remove" => {
            let file_path = String::from(input.next().unwrap());
            user_interaction::remove_in(file_path);
        },
        "status" => {
            let file_path = String::from(input.next().unwrap());
            user_interaction::status_in(file_path);
        },
        _ => {
            println!("Invalid command");
        }
    }

}
