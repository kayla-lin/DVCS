pub mod user_feedback{

    use colored::Colorize;
    use rand::prelude::*;
    use std::io;
    use std::io::prelude::*;
    
    pub fn display_all_errors(errors: Vec<String>) -> bool {
            //errors to iterator
            let error_inter: Vec<String> =  errors.into_iter().collect();
            let mut rng = rand::thread_rng();
            
            //display errors
            error_inter.iter().for_each(|error:&String| println!("{}", error.truecolor(rng.gen(), rng.gen(), rng.gen())));
            return true;
        }
    
    pub fn display_first_error(errors: Vec<String>) -> bool {
            println!("{}", errors[0].red());
            return true;
        }
        
        
    pub fn format_error_alt(errors: Vec<String>) -> bool{
            //print every 3 errors and display more when user enters
            fn pause() {
                let mut stdin = io::stdin();
                let mut stdout = io::stdout();
            
                write!(stdout, "Press any key to continue...").unwrap();
                stdout.flush().unwrap();
            
                // Read a single byte and discard
                let _ = stdin.read(&mut [0u8]).unwrap();
            }

            let  mut rng = rand::thread_rng();
        
            errors.chunks(3).for_each(|error| {
                
                error.to_vec().iter().for_each(|error| println!("{}", error.truecolor(rng.gen(), rng.gen(), rng.gen())));
                pause();
            });
            return true;
    
        }
    
    
    }


pub mod user_interaction{

    use stager::stager::Stager;
    use std::{path::Path, collections::HashMap};
    use crate::user_feedback::{display_first_error, display_all_errors, format_error_alt};
    use storage_hiding::repository_storage::{RepositoryStorage}; 
    use repo_directory_hiding::merge_states; 
    use repo_directory_hiding::State; 

    pub fn init_in(file_path: String) -> bool {
        //let res = Path::new(&file_path).try_exists().unwrap_or_else(|_| false); 
        match Path::new(&file_path).try_exists().unwrap_or_else(|_| false) {
            true => {
                let fp = file_path.clone();
                let stager_i = Stager::new("DVCS_HIDDEN", fp.as_str());
                if stager_i.is_err() {
                    let t_er = stager_i.unwrap_err();
                    display_first_error(vec![t_er]);
                    return false;
                }

                let init_res = stager_i.unwrap().init(file_path);
                if init_res.is_err(){
                    let t_er = init_res.unwrap_err();
                    display_first_error(vec![t_er]);
                    return false;
                }
                println!("Initialized empty dvcs repository in {}", &fp);
                return true;
            },
            false => {
                println!("Error!");
                println!("Which error function do you want to use?");
                //read console input
                //match input to error function
                let errchoice = std::io::stdin();
                println!("\n1. Display first error\n2. Display all errors\n3. Display errors in chunks\n");
                let mut input = String::new();
                let errfn = errchoice.read_line(&mut input).unwrap();
        
                let t_er = "file path: ".to_owned() + &file_path.to_string() + " does not exists!";
                match errfn {
                    1 => {
                        //display first error
                        display_first_error(vec![t_er]);
                        return false;
                    }
                    2 => {
                        //display all errors
                        display_all_errors(vec![t_er]);
                        return false;
                    }
                    3 => {
                        //display errors in chunks
                        format_error_alt(vec![t_er]);
                        return false;
                    }
                    _ => {
                        //display first error
                    display_all_errors(vec![t_er]);
                    return false;
                    },
                }
            },
        }
    }


    pub fn diff_in(file_path: String, head: String) -> bool{
        let res: bool = Path::new(&file_path).try_exists().unwrap_or_else(|_| false);
        match res {
            true => {
                println!("File exists, diffing...");

                let stager_i = Stager::new("DVCS_HIDDEN", file_path.as_str());
                if stager_i.is_err() {
                    let t_er = stager_i.unwrap_err();
                    display_first_error(vec![t_er]);
                    return false;
                }
                let diff_res = stager_i.unwrap().diff(file_path, head);
                if diff_res.is_err(){
                    let t_er = diff_res.unwrap_err();
                    display_first_error(vec![t_er]);
                    return false;
                }
                return true;
            },
            false => {
                println!("Error!");
                println!("Which error function do you want to use?");
                //read console input
                //match input to error function

                let errchoice = std::io::stdin();
                println!("\n1. Display first error\n2. Display all errors\n3. Display errors in chunks\n");
                let mut input = String::new();
                let errfn = errchoice.read_line(&mut input).unwrap();
        
                let t_er = "file path: ".to_owned() + &file_path.to_string() + " does not exists!";
                match errfn {
                    1 => {
                        //display first error
                        display_first_error(vec![t_er]);
                        return false;
                    }
                    2 => {
                        //display all errors
                        display_all_errors(vec![t_er]);
                        return false;
                    }
                    3 => {
                        //display errors in chunks
                        format_error_alt(vec![t_er]);
                        return false;
                    }
                    _ => {
                        //display first error
                    display_all_errors(vec![t_er]);
                    return false;
                    },
                }
            }
        }
    }
    pub fn status_in(file_path: String) -> bool{
        let res: bool = Path::new(&file_path).try_exists().unwrap_or_else(|_| false);
        match res {
            true => {
                println!("File exists, status...");

                let stager_i = Stager::new("DVCS_HIDDEN", file_path.as_str());
                if stager_i.is_err() {
                    let t_er = stager_i.unwrap_err();
                    display_first_error(vec![t_er]);
                    return false;
                }
                let status_res = stager_i.unwrap().status(file_path);

                if status_res.is_ok() {
                    print!("{}", status_res.unwrap());
                    return true;
                }else {
                    println!("Error! File path empty");
                    return false;
                }
            },
            false => {
                println!("Error!");
                println!("Which error function do you want to use?");
                //read console input
                //match input to error function

                let errchoice = std::io::stdin();
                println!("\n1. Display first error\n2. Display all errors\n3. Display errors in chunks\n");
                let mut input = String::new();
                let errfn = errchoice.read_line(&mut input).unwrap();
        
                let t_er = "file path: ".to_owned() + &file_path.to_string() + " does not exists!";
                match errfn {
                    1 => {
                        //display first error
                        display_first_error(vec![t_er]);
                        return false;
                    }
                    2 => {
                        //display all errors
                        display_all_errors(vec![t_er]);
                        return false;
                    }
                    3 => {
                        //display errors in chunks
                        format_error_alt(vec![t_er]);
                        return false;
                    }
                    _ => {
                        //display first error
                    display_all_errors(vec![t_er]);
                    return false;
                    },
                }
            }
        }

    }

    pub fn remove_in(file_path: String) -> bool{    
        let res: bool = Path::new(&file_path).try_exists().unwrap_or_else(|_| false);
        if res {
            println!("File exists, removing...");

            let stager_i = Stager::new("DVCS_HIDDEN", file_path.as_str());
            if stager_i.is_err() {
                let t_er = stager_i.unwrap_err();
                display_first_error(vec![t_er]);
                return false;
            }
            let remove_res = stager_i.unwrap().remove(file_path);
            if remove_res.is_ok(){
                return true;
            }else {
                let err = remove_res.unwrap_err();
                display_first_error(vec![err]);
                return false;
            }
        }
        return true; 
    }

    pub fn add_in(file_path: String) -> bool{
        let res: bool = Path::new(&file_path).try_exists().unwrap_or_else(|_| false);
        if res {
            println!("File exists, adding...");

            let stager_i = Stager::new("DVCS_HIDDEN", file_path.as_str());
            if stager_i.is_err() {
                let t_er = stager_i.unwrap_err();
                display_first_error(vec![t_er]);
                return false;
            }
            let add_res = stager_i.unwrap().add(file_path);
            if add_res.is_ok(){
                return true;
            }else {
                let err = add_res.unwrap_err();
                display_first_error(vec![err]);
                return false;
            }
        }
        return true; 
    }

    pub fn see_diff_in(snapshot: &HashMap<String, String>) -> (HashMap<String, String>, bool){
        //new instance of RepositoryStorage
        let repo = RepositoryStorage::new();
        (repo.see_diff(snapshot), true)
    }
    



}

mod err_handling_tests{
    use crate::user_feedback::*;

    #[test]
    fn test_display_first_error(){
        let test_err = vec!["test error on first err fn".to_string()];
        let res = display_first_error(test_err);
        assert_eq!(res, true);
    }
    #[test]
    fn test_display_all_errors(){
        let test_err = vec!["test error on display all err fn".to_string()];
        let res = display_all_errors(test_err);
        assert_eq!(res, true);
    }
    #[test]
    fn test_format_error_alt(){
        print!("Press any key to continue...\n"); 
        let test_err = vec!["test error on format err fn".to_string()];
        let res = format_error_alt(test_err);
        assert_eq!(res, true);
    }


}

mod user_interaction_tests{
    use crate::user_interaction::diff_in;
    use crate::user_interaction::init_in;
    use crate::user_interaction::status_in;
    use crate::user_interaction::remove_in;
    use crate::user_interaction::add_in;
    
    

    
    static mut TEST_PATH: &str = "/tmp/dvcs_test/";
    #[test]
    fn init_test_succ(){
        //create a valid path
        let test_path = "/tmp/dvcs_test/";
        let res = init_in(test_path.to_string());
        assert_eq!(res, true);
        
    }
    #[test]
    fn init_test_fail(){
        //create an invalid path
        let test_path = "/tmp/dvcs_st";
        let res = init_in(test_path.to_string());
        assert_eq!(res, false);
    }
    #[test]

    fn diff_in_test(){
        let test_path = "/tmp/dvcs_test/";
        let res = diff_in(test_path.to_string(), "HEAD".to_string());
        assert_eq!(res, true);
    }
    #[test]
    fn status_in_test(){
        let test_path = "/tmp/dvcs_test/";
        let res = status_in(test_path.to_string());
        assert_eq!(res, true);
    }

    #[test]
    fn remove_in_test(){
        let test_path = "/tmp/dvcs_test/";
        let res = remove_in(test_path.to_string());
        assert_eq!(res, true);
    }
    #[test]
    fn add_in_test(){
        let test_path = "/tmp/dvcs_test/";
        let res = add_in(test_path.to_string());
        assert_eq!(res, true);
    }

    
}


