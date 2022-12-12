use std::collections::HashMap;
use std::fs::File;
use std::io::Stdin;
use std::panic::catch_unwind;
use std::{fs, io};
use storage_hiding::repository_storage::{self, RepositoryStorage};
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
    use std::panic;

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

    //level 3 status, heads, cat, log
    #[test]
    fn status_test_3a(){
        //pass
        const DVCS_HIDDEN: &str = "/tmp/dvcs_team";
        let file_path = String::from("/tmp/dvcs_testi/");
        

        std::fs::create_dir_all(DVCS_HIDDEN);
        std::fs::create_dir_all("/tmp/dvcs_testi/");
        
        let status_res = panic::catch_unwind(|| {user_interaction::status_in(file_path)});
        assert_eq!(status_res.unwrap_or(true), true);
    }

    #[test]
    fn status_test_3b(){
        //fails
        const DVCS_HIDDEN: &str = "/tmp/dvcs_team";
        let file_path = String::from("/tmp//");

        std::fs::create_dir_all(DVCS_HIDDEN);
        std::fs::create_dir_all("/tmp/dvcs_testi/");
        let status_res = panic::catch_unwind(|| {user_interaction::status_in(file_path)});
        assert_eq!(status_res.unwrap_or(false), true);
    }

    //level 4 diff, remove 
    #[test]
    fn diff_test_4a(){
        //pass
        const DVCS_HIDDEN: &str = "/tmp/dvcs_team";
        let file_path = String::from("/tmp/dvcs_testi/");
        let head = String::from("HEAD");

        std::fs::create_dir_all(DVCS_HIDDEN);
        std::fs::create_dir_all("/tmp/dvcs_testi/");
        
        let diff_res = panic::catch_unwind(|| {user_interaction::diff_in(file_path, head)});
        assert_eq!(diff_res.unwrap_or(true), true);
    }
    #[test]
    fn diff_test_4b(){
        //fails 
        const DVCS_HIDDEN: &str = "/tmp/dvcs_team";
        let file_path = String::from("/tmp/dvcsesti/");
        let head = String::from("HEAD");

        std::fs::create_dir_all(DVCS_HIDDEN);
        std::fs::create_dir_all("/tmp/dvcs_testi/");
        
        let diff_res = panic::catch_unwind(|| {user_interaction::diff_in(file_path, head)});
        assert_eq!(diff_res.unwrap_or(true), false);
    }
    #[test]
    fn remove_test_4c(){
        //pass
        const DVCS_HIDDEN: &str = "/tmp/dvcs_team";
        let file_path = String::from("/tmp/dvcs_testi/");

        std::fs::create_dir_all(DVCS_HIDDEN);
        std::fs::create_dir_all("/tmp/dvcs_testi/");
        
        let remove_res = panic::catch_unwind(|| {user_interaction::remove_in(file_path)});
        assert_eq!(remove_res.unwrap_or(true), true);
    }
    #[test]
    fn remove_test_4d(){
        //fails
        const DVCS_HIDDEN: &str = "/tmp/dvcs_team";
        let file_path = String::from("/tmp/");

        std::fs::create_dir_all(DVCS_HIDDEN);
        std::fs::create_dir_all("/tmp/dvcs_testi/");
        
        let remove_res = panic::catch_unwind(|| {user_interaction::remove_in(file_path)});
        assert_eq!(remove_res.unwrap_or(true), false);
    }


}
