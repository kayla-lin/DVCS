use std::fs;
use staging::*;
use usemods::{*, user_interaction::init_in};
use std::fs::File;


fn main() {
    println!("Hello, world!");
    const DVCS_HIDDEN:&str = "/tmp/dvcs_team";
    fs::create_dir(DVCS_HIDDEN);
    fs::create_dir("/tmp/dvcs_testi/");
    let file = File::create("/tmp/dvcs_testi/");
    
   
}
