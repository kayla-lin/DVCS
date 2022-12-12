use std::collections::HashMap;
use std::fs::File;
use std::io::Stdin;
use std::panic::catch_unwind;
use std::{fs, io};
use storage_hiding::repository_storage::{self, RepositoryStorage};
use usemods::dir_c::create_dir_main;
use usemods::*;

fn main() {
    const DVCS_HIDDEN: &str = "/tmp/dvcs_team";

    fs::create_dir(DVCS_HIDDEN);
    fs::create_dir("/tmp/dvcs_testi/");
    let _sfile = File::create("/tmp/dvcs_testi/");
    //init -> new instance of a repository in the current directory
    //diff -> see the differences between the current repository structure and the previous snapshot
    //status -> see the current repository structure
    //add -> add a file to the repository
    //remove -> remove a file from the repository

    //new -> new RepositoryStorage instance

    //taking user input
    println!("Welcome to the DVCS:\n ");
    let mut input = String::new();

    let stdin: Stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    let mut input = input.split_whitespace();
    let command = String::from(input.next().unwrap());
    let mut repo_storage: RepositoryStorage;

    match command.as_str() {
        "init" => {
            let file_path = String::from(input.next().unwrap_or_else(|| {
                println!("Empty path");
                ""
            }));
            user_interaction::init_in(file_path);
        }
        "diff" => {
            let file_path = String::from(input.next().unwrap_or_else(|| {
                println!("Empty path");
                ""
            }));
            let head: String = String::from(input.next().unwrap_or_else(|| {
                println!("Empty head");
                ""
            }));
            user_interaction::diff_in(file_path, head);
        }
        "status" => {
            let file_path = String::from(input.next().unwrap_or_else(|| {
                println!("Empty path");
                ""
            }));
            user_interaction::status_in(file_path);
        }
        "add" => {
            let file_path = String::from(input.next().unwrap_or_else(|| {
                println!("Empty path");
                ""
            }));
            user_interaction::add_in(file_path);
        }
        "remove" => {
            let file_path = String::from(input.next().unwrap_or_else(|| {
                println!("Empty path");
                ""
            }));
            user_interaction::remove_in(file_path);
        }
        "see_diff" => {
            //let snapshot =
            let diff = user_interaction::see_diff_in(&HashMap::new()).0;
        }

        _ => {
            println!("Invalid command");
        }
    }
}


mod acceptance_tests{
    use staging::staging_storage::Staging;
    use usemods::user_interaction;

    //level 1 init -> new instance of a repository in the current directory
    #[test]
    fn init_test_1a(){

        const DVCS_HIDDEN: &str = "/tmp/dvcs_team";
        let file_path = String::from("/tmp/dvcs_testi/");
        
        std::fs::create_dir_all(DVCS_HIDDEN);
        std::fs::create_dir_all("/tmp/dvcs_testi/");
        let init_res = user_interaction::init_in(file_path);
        assert_eq!(init_res, true);
        
    }
    #[test]
    fn init_test_1b(){
        //fails 
        const DVCS_HIDDEN: &str = "/tmp/dvcs_team";
        let file_path = String::from("/tmp/dvcs_ti/");

        std::fs::create_dir_all(DVCS_HIDDEN);
        std::fs::create_dir_all("/tmp/dvcs_testi/");
        let init_res = user_interaction::init_in(file_path);
        assert_eq!(init_res, false);
    }

    //level 2 add -> add a file to the repository, commit -> commit the current state of the repository, checkout -> checkout a previous commit
    #[test]
    fn add_test_2a(){
        const DVCS_HIDDEN: &str = "/tmp/dvcs_team";
        let file_path = String::from("/tmp/dvcs_testi/");
        

        std::fs::create_dir_all(DVCS_HIDDEN);
        std::fs::create_dir_all("/tmp/dvcs_testi/");
        
        let add_res = user_interaction::add_in(file_path);
        assert_eq!(add_res, true);
    }
    #[test]
    fn add_test_2b(){
        //fails
        const DVCS_HIDDEN: &str = "/tmp/dvcs_team";
        let file_path = String::from("/tmp//");

        std::fs::create_dir_all(DVCS_HIDDEN);
        std::fs::create_dir_all("/tmp/dvcs_testi/");
        let add_res = user_interaction::add_in(file_path);
        assert_eq!(add_res, false);
    }
}