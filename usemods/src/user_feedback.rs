
    
    use colored::Colorize;
    //display all errors
    use rand::prelude::*;
    extern crate colored;
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
    //function for missing arguments 
    pub fn format_error_alt(errors: Vec<String>) {
        //#print every 3 errors and display more when user enters
        
        let mut length = errors.len();
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


