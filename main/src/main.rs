use staging::*;
use std::fs;
use std::fs::File;
use usemods::{user_interaction::init_in, *};

fn main() {
    println!("Hello, world!");
    const DVCS_HIDDEN: &str = "/tmp/dvcs_team";
    fs::create_dir(DVCS_HIDDEN);
    fs::create_dir("/tmp/dvcs_testi/");
    let file = File::create("/tmp/dvcs_testi/");
}
