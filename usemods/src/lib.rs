pub mod user_feedback{

    use colored::Colorize;
    use rand::prelude::*;
    use std::io;
    use std::io::prelude::*;
    
    pub fn display_all_errors(errors: Vec<String>) {
            //errors to iterator
            let error_inter: Vec<String> =  errors.into_iter().collect();
            let mut rng = rand::thread_rng();
            
            //display errors
            error_inter.iter().for_each(|error:&String| println!("{}", error.truecolor(rng.gen(), rng.gen(), rng.gen())));
        
        }
    
    pub fn display_first_error(errors: Vec<String>) {
            println!("{}", errors[0].red());
        }
        
        
    pub fn format_error_alt(errors: Vec<String>) {
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
    
        }
    
    
    }


pub mod user_interaction{

    use stager;
    use std::path::Path;

    use crate::user_feedback::{display_first_error, display_all_errors, format_error_alt};
    
    pub fn init_in(file_path: String) -> bool {
        //let res = Path::new(&file_path).try_exists().unwrap_or_else(|_| false); 
        match Path::new(&file_path).try_exists().unwrap_or_else(|_| false) {
            true => {
                println!("File exists, initializing..."); 
                stager::stager::Stager::init(file_path);
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
                stager::stager::Stager::diff(file_path, head);
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
                let status_res = stager::stager::Stager::status(file_path);

                if status_res.is_ok(){
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
        return stager::stager::Stager::remove(file_path).is_ok();
    }

    pub fn add_in(file_path: String) -> bool{
        let res = stager::stager::Stager::add(file_path);
        return res.is_ok();
    }

}

pub mod dir_c{
    use std::{fs, io};
    use std::fs::File;

    pub fn create_dir_main() -> io::Result<()> {
        const DVCS_HIDDEN: &str = "/tmp/dvcs_team";
   
        fs::create_dir(DVCS_HIDDEN)?;
        fs::create_dir("/tmp/dvcs_testi/")?;
        let file = File::create("/tmp/dvcs_testi/");
        Ok(())
    }
}




